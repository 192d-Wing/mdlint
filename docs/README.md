# mkdlint Rules Documentation

This directory contains detailed documentation for all 64 mkdlint rules.

## Standard Rules (MD001-MD060)

| Rule | Name | Description | Fixable |
|------|------|-------------|---------|
| [MD001](md001.md) | heading-increment | Heading levels should only increment by one level at a time | ✓ |
| [MD003](md003.md) | heading-style | Heading style should be consistent | ✓ |
| [MD004](md004.md) | ul-style | Unordered list style should be consistent | ✓ |
| [MD005](md005.md) | list-indent | Inconsistent indentation for list items at the same level | Partial |
| [MD007](md007.md) | ul-indent | Unordered list indentation should be consistent | ✓ |
| [MD009](md009.md) | no-trailing-spaces | Trailing spaces | ✓ |
| [MD010](md010.md) | no-hard-tabs | Hard tabs | ✓ |
| [MD011](md011.md) | no-reversed-links | Reversed link syntax | ✓ |
| [MD012](md012.md) | no-multiple-blanks | Multiple consecutive blank lines | ✓ |
| [MD013](md013.md) | line-length | Line length | ✗ |
| [MD014](md014.md) | commands-show-output | Dollar signs used before commands without showing output | ✓ |
| [MD018](md018.md) | no-missing-space-atx | No space after hash on atx style heading | ✓ |
| [MD019](md019.md) | no-multiple-space-atx | Multiple spaces after hash on atx style heading | ✓ |
| [MD020](md020.md) | no-missing-space-closed-atx | No space inside hashes on closed atx style heading | ✓ |
| [MD021](md021.md) | no-multiple-space-closed-atx | Multiple spaces inside hashes on closed atx style heading | ✓ |
| [MD022](md022.md) | blanks-around-headings | Headings should be surrounded by blank lines | ✓ |
| [MD023](md023.md) | heading-start-left | Headings must start at the beginning of the line | ✓ |
| [MD024](md024.md) | no-duplicate-heading | Multiple headings with the same content | ✗ |
| [MD025](md025.md) | single-title | Multiple top-level headings in the same document | ✗ |
| [MD026](md026.md) | no-trailing-punctuation | Trailing punctuation in heading | ✓ |
| [MD027](md027.md) | no-multiple-space-blockquote | Multiple spaces after blockquote symbol | ✓ |
| [MD028](md028.md) | no-blanks-blockquote | Blank line inside blockquote | ✗ |
| [MD029](md029.md) | ol-prefix | Ordered list item prefix | ✓ |
| [MD030](md030.md) | list-marker-space | Spaces after list markers | ✓ |
| [MD031](md031.md) | blanks-around-fences | Fenced code blocks should be surrounded by blank lines | ✓ |
| [MD032](md032.md) | blanks-around-lists | Lists should be surrounded by blank lines | ✓ |
| [MD033](md033.md) | no-inline-html | Inline HTML | ✗ |
| [MD034](md034.md) | no-bare-urls | Bare URL used | ✓ |
| [MD035](md035.md) | hr-style | Horizontal rule style | ✓ |
| [MD036](md036.md) | no-emphasis-as-heading | Emphasis used instead of a heading | ✗ |
| [MD037](md037.md) | no-space-in-emphasis | Spaces inside emphasis markers | ✓ |
| [MD038](md038.md) | no-space-in-code | Spaces inside code span elements | ✓ |
| [MD039](md039.md) | no-space-in-links | Spaces inside link text | ✓ |
| [MD040](md040.md) | fenced-code-language | Fenced code blocks should have a language specified | ✓ |
| [MD041](md041.md) | first-line-heading | First line in a file should be a top-level heading | ✓ |
| [MD042](md042.md) | no-empty-links | No empty links | ✓ |
| [MD043](md043.md) | required-headings | Required heading structure | ✗ |
| [MD044](md044.md) | proper-names | Proper names should have the correct capitalization | ✓ |
| [MD045](md045.md) | no-alt-text | Images should have alternate text (alt text) | ✓ |
| [MD046](md046.md) | code-block-style | Code block style | ✓ |
| [MD047](md047.md) | single-trailing-newline | Files should end with a single newline character | ✓ |
| [MD048](md048.md) | code-fence-style | Code fence style | ✓ |
| [MD049](md049.md) | emphasis-style | Emphasis style | ✓ |
| [MD050](md050.md) | strong-style | Strong style | ✓ |
| [MD051](md051.md) | link-fragments | Link fragments should be valid | ✗ |
| [MD052](md052.md) | reference-links-images | Reference links and images should use a label that is defined | ✓ |
| [MD053](md053.md) | link-image-reference-definitions | Link and image reference definitions should be needed | ✓ |
| [MD054](md054.md) | link-image-style | Link and image style | ✓ |
| [MD055](md055.md) | table-pipe-style | Table pipe style | ✓ |
| [MD056](md056.md) | table-column-count | Table column count | ✗ |
| [MD058](md058.md) | table-separator-style | Table separator style | ✓ |
| [MD059](md059.md) | emphasis-markers | Emphasis markers should match configured style | ✓ |
| [MD060](md060.md) | dollar-in-code-fence | Dollar signs in fenced code blocks | ✓ |

## Kramdown Extension Rules (KMD001-KMD011)

| Rule | Name | Description | Fixable |
|------|------|-------------|---------|
| [KMD001](kmd001.md) | kramdown-definition-lists | Kramdown definition list syntax | ✗ |
| [KMD002](kmd002.md) | kramdown-footnotes | Kramdown footnote syntax | ✓ |
| [KMD003](kmd003.md) | kramdown-abbreviations | Kramdown abbreviation syntax | ✓ |
| [KMD004](kmd004.md) | kramdown-link-attributes | Kramdown link attributes syntax | ✓ |
| [KMD005](kmd005.md) | kramdown-duplicate-heading-ids | Duplicate heading IDs in Kramdown | ✓ |
| [KMD006](kmd006.md) | kramdown-ial-syntax | Invalid Kramdown IAL syntax | ✓ |
| [KMD007](kmd007.md) | kramdown-block-ial-spacing | Block IAL spacing | ✗ |
| [KMD008](kmd008.md) | kramdown-extension-blocks | Kramdown extension blocks | ✓ |
| [KMD009](kmd009.md) | kramdown-ald-syntax | Kramdown ALD syntax | ✓ |
| [KMD010](kmd010.md) | kramdown-blank-line-before-blocks | Blank lines before block elements | ✗ |
| [KMD011](kmd011.md) | kramdown-math-blocks | Kramdown math block syntax | ✓ |

## Legend

- ✓ = Auto-fixable with `--fix`
- ✗ = Not auto-fixable
- Partial = Partially auto-fixable (see rule doc for details)

## See Also

- [Upstream markdownlint documentation](https://github.com/DavidAnson/markdownlint/blob/main/doc/Rules.md)
- [Configuration guide](../README.md#configuration)
- [Contributing guide](../CONTRIBUTING.md)
