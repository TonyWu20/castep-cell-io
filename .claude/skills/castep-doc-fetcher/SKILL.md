---
name: castep-doc-fetcher
description: Fetch CASTEP documentation and generate TODOS.md for unimplemented keywords
---

# CASTEP Documentation Fetcher

Fetches keyword documentation from CASTEP's official web documentation, compares with current codebase, and generates `TODOS.md` with unimplemented keywords classified by pattern and grouped by documentation category.

## Instructions

Run the embedded Python script with `uv run python` to generate TODOS.md in workspace root.

## Python Script

```python
#!/usr/bin/env python3
import sys, subprocess
try:
    import requests
    from bs4 import BeautifulSoup
    import markdownify
except ImportError:
    subprocess.run([sys.executable, '-m', 'pip', 'install', 'requests', 'beautifulsoup4', 'markdownify'], check=True)
    import requests
    from bs4 import BeautifulSoup
    import markdownify
import re, os
from collections import defaultdict
from pathlib import Path

CELL_DOC_URL = "https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_main_structure.htm"
PARAM_DOC_URL = "https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_main_parameters.htm"
BASE_URL = "https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/"

def fetch_keywords_from_doc(url):
    response = requests.get(url)
    soup = BeautifulSoup(response.content, 'html.parser')
    keywords, current_group = [], "Uncategorized"
    for elem in soup.find_all(['h4', 'a']):
        if elem.name == 'h4':
            current_group = elem.get_text(strip=True)
        elif elem.name == 'a':
            name = elem.get_text(strip=True)
            if name.isupper() and len(name) > 3 and (href := elem.get('href', '')):
                kw_info = fetch_keyword_details(name, BASE_URL + href, current_group)
                kw_info['group'] = current_group
                keywords.append(kw_info)
    return keywords

def fetch_keyword_details(name, url, group):
    try:
        response = requests.get(url, timeout=5)
        soup = BeautifulSoup(response.content, 'html.parser')
        text = soup.get_text()
        type_match = re.search(r'Keyword type\s+(\w+)', text, re.IGNORECASE) or re.search(r'Type:\s*(\w+)', text, re.IGNORECASE)
        default_match = re.search(r'Default[:\s]+([^\n]+)', text, re.IGNORECASE)
        has_options = bool(re.search(r'Allow(?:ed)?\s+(?:options|values|choices)', text, re.IGNORECASE))
        
        # Save full documentation as markdown
        save_keyword_doc(name, url, soup, group)
        
        return {'name': name, 'type': type_match.group(1) if type_match else 'Unknown',
                'default': default_match.group(1).strip() if default_match else '',
                'description': text[:500].replace('\n', ' '), 'has_options': has_options, 'url': url}
    except Exception as e:
        print(f"   Warning: Failed to fetch {name}: {e}")
        return {'name': name, 'type': 'Unknown', 'default': '', 'description': '', 'has_options': False, 'url': url}

def save_keyword_doc(name, url, soup, group):
    """Save keyword documentation page as markdown"""
    # Create directory structure
    safe_group = re.sub(r'[^\w\s-]', '', group).strip().replace(' ', '_')
    doc_dir = Path('KEYWORD_DOCS') / safe_group
    doc_dir.mkdir(parents=True, exist_ok=True)
    
    # Extract main content (skip navigation/header/footer)
    content = soup.find('div', class_='body') or soup.find('main') or soup.find('article') or soup.body
    if content:
        # Convert to markdown
        md_content = markdownify.markdownify(str(content), heading_style="ATX")
        
        # Add header with metadata
        header = f"# {name}\n\n**Source:** {url}\n\n**Group:** {group}\n\n---\n\n"
        
        # Save to file
        filename = doc_dir / f"{name}.md"
        with open(filename, 'w', encoding='utf-8') as f:
            f.write(header + md_content)

def scan_implemented_keywords():
    implemented = set()
    result = subprocess.run(['fd', '-e', 'rs', '.', 'castep_cell_data/src/cell', 
                            'castep_cell_data/src/param', 'castep_cell_data/src/units'],
                           capture_output=True, text=True)
    for file_path in result.stdout.strip().split('\n'):
        if file_path:
            try:
                content = open(file_path, 'r').read()
                implemented.update(k.upper() for k in re.findall(r'const KEY_NAME:\s*&.*?=\s*"([^"]+)"', content))
                implemented.update(b.upper() for b in re.findall(r'const BLOCK_NAME:\s*&.*?=\s*"([^"]+)"', content))
                implemented.update(u.upper() for u in re.findall(r'#\[serde\(rename\s*=\s*"([^"]+)"\)\][^\n]*\s*pub enum', content))
            except:
                pass
    return implemented

def classify_keyword(name, type_str, description, has_options):
    desc_upper, type_upper = description.upper(), type_str.upper()
    if type_str == 'Block' or '%BLOCK' in desc_upper or 'BLOCK' in name:
        return 'Pattern 4: Block Type'
    if '|' in type_str or has_options or 'ALLOW' in desc_upper:
        return 'Pattern 1: Simple Keyword Enum'
    if 'UNIT' in type_upper or '+' in type_str:
        return 'Pattern 3: Composite Key-Value Type'
    if type_str in ['Real', 'Integer', 'String', 'Logical', 'Physical']:
        return 'Pattern 2: Simple Scalar Type'
    return 'Pattern 2: Simple Scalar Type'

def generate_todos(cell_keywords, param_keywords, implemented):
    cell_missing = defaultdict(lambda: defaultdict(list))
    param_missing = defaultdict(lambda: defaultdict(list))
    for kw in cell_keywords:
        if kw['name'].upper() not in implemented:
            pattern = classify_keyword(kw['name'], kw['type'], kw['description'], kw.get('has_options', False))
            cell_missing[kw.get('group', 'Uncategorized')][pattern].append(kw)
    for kw in param_keywords:
        if kw['name'].upper() not in implemented:
            pattern = classify_keyword(kw['name'], kw['type'], kw['description'], kw.get('has_options', False))
            param_missing[kw.get('group', 'Uncategorized')][pattern].append(kw)
    total_cell, total_param = len(cell_keywords), len(param_keywords)
    missing_cell = sum(sum(len(v) for v in p.values()) for p in cell_missing.values())
    missing_param = sum(sum(len(v) for v in p.values()) for p in param_missing.values())
    with open('TODOS.md', 'w') as f:
        f.write(f'# CASTEP Implementation TODOs\n\n## Summary\n\n')
        f.write(f'- Total cell: {total_cell}, Implemented: {total_cell - missing_cell}, Missing: {missing_cell}\n')
        f.write(f'- Total param: {total_param}, Implemented: {total_param - missing_param}, Missing: {missing_param}\n\n')
        if cell_missing:
            f.write('## Missing Cell Keywords\n\n')
            for group in sorted(cell_missing.keys()):
                f.write(f'### Group: {group}\n\n')
                for pattern in sorted(cell_missing[group].keys()):
                    f.write(f'#### {pattern}\n\n')
                    for kw in cell_missing[group][pattern]:
                        f.write(f"- **{kw[\'name\']}** (Type: {kw[\'type\']}, Default: {kw[\'default\']}, URL: {kw.get(\'url\', \'\')})
")
                f.write('\n')
        if param_missing:
            f.write('## Missing Param Keywords\n\n')
            for group in sorted(param_missing.keys()):
                f.write(f'### Group: {group}\n\n')
                for pattern in sorted(param_missing[group].keys()):
                    f.write(f'#### {pattern}\n\n')
                    for kw in param_missing[group][pattern]:
                        f.write(f"- **{kw[\'name\']}** (Type: {kw[\'type\']}, Default: {kw[\'default\']}, URL: {kw.get(\'url\', \'\')})
")
                f.write('\n')
    return missing_cell, missing_param

def main():
    print("CASTEP Documentation Fetcher\n" + "=" * 50)
    print("\n1. Fetching cell keywords and saving documentation...")
    cell_keywords = fetch_keywords_from_doc(CELL_DOC_URL)
    print(f"   Found {len(cell_keywords)} cell keywords")
    print("\n2. Fetching param keywords and saving documentation...")
    param_keywords = fetch_keywords_from_doc(PARAM_DOC_URL)
    print(f"   Found {len(param_keywords)} param keywords")
    print("\n3. Scanning implemented keywords...")
    implemented = scan_implemented_keywords()
    print(f"   Found {len(implemented)} implemented keywords")
    print("\n4. Generating TODOS.md...")
    missing_cell, missing_param = generate_todos(cell_keywords, param_keywords, implemented)
    print("\n" + "=" * 50)
    print(f"COMPLETE!\nMissing: {missing_cell} cell, {missing_param} param ({missing_cell + missing_param} total)")
    print(f"Documentation saved to KEYWORD_DOCS/ (organized by group)")

if __name__ == '__main__':
    main()
```

## Usage

Run `/castep-doc-fetcher` or execute the script with `uv run python` to generate TODOS.md.

## Notes

- Detects unit enums in `castep_cell_data/src/units/`
- Groups keywords by documentation categories
- String-type keywords may need manual review as they're often enums with limited options
- Full keyword documentation pages are saved as markdown in `KEYWORD_DOCS/` organized by group subdirectories
- Each markdown file includes metadata header with source URL and group classification
