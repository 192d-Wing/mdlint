# mkdlint Rules Documentation

Per-rule documentation lives in the [rules/](rules/) subdirectory.

## Standard Rules (MD001-MD060)

| Rule | Name | Description | Fixable |
|------|------|-------------|---------|
| [MD001](rules/md001.md) | heading-increment | Heading levels should only increment by one level at a time | ✗ |
| [MD003](rules/md003.md) | heading-style | Heading style should be consistent | ✓ |
| [MD004](rules/md004.md) | ul-style | Unordered list style should be consistent | ✓ |
| [MD005](rules/md005.md) | list-indent | Inconsistent indentation for list items at the same level | Partial |
| [MD007](rules/md007.md) | ul-indent | Unordered list indentation should be consistent | ✓ |
| [MD009](rules/md009.md) | no-trailing-spaces | Trailing spaces | ✓ |
| [MD010](rules/md010.md) | no-hard-tabs | Hard tabs | ✓ |
| [MD011](rules/md011.md) | no-reversed-links | Reversed link syntax | ✓ |
| [MD012](rules/md012.md) | no-multiple-blanks | Multiple consecutive blank lines | ✓ |
| [MD013](rules/md013.md) | line-length | Line length | ✗ |
| [MD014](rules/md014.md) | commands-show-output | Dollar signs used before commands without showing output | ✓ |
| [MD018](rules/md018.md) | no-missing-space-atx | No space after hash on atx style heading | ✓ |
| [MD019](rules/md019.md) | no-multiple-space-atx | Multiple spaces after hash on atx style heading | ✓ |
| [MD020](rules/md020.md) | no-missing-space-closed-atx | No space inside hashes on closed atx style heading | ✓ |
| [MD021](rules/md021.md) | no-multiple-space-closed-atx | Multiple spaces inside hashes on closed atx style heading | ✓ |
| [MD022](rules/md022.md) | blanks-around-headings | Headings should be surrounded by blank lines | ✓ |
| [MD023](rules/md023.md) | heading-start-left | Headings must start at the beginning of the line | ✓ |
| [MD024](rules/md024.md) | no-duplicate-heading | Multiple headings with the same content | ✗ |
| [MD025](rules/md025.md) | single-title | Multiple top-level headings in the same document | ✗ |
| [MD026](rules/md026.md) | no-trailing-punctuation | Trailing punctuation in heading | ✓ |
| [MD027](rules/md027.md) | no-multiple-space-blockquote | Multiple spaces after blockquote symbol | ✓ |
| [MD028](rules/md028.md) | no-blanks-blockquote | Blank line inside blockquote | ✓ |
| [MD029](rules/md029.md) | ol-prefix | Ordered list item prefix | ✓ |
| [MD030](rules/md030.md) | list-marker-space | Spaces after list markers | ✓ |
| [MD031](rules/md031.md) | blanks-around-fences | Fenced code blocks should be surrounded by blank lines | ✓ |
| [MD032](rules/md032.md) | blanks-around-lists | Lists should be surrounded by blank lines | ✓ |
| [MD033](rules/md033.md) | no-inline-html | Inline HTML | ✗ |
| [MD034](rules/md034.md) | no-bare-urls | Bare URL used | ✓ |
| [MD035](rules/md035.md) | hr-style | Horizontal rule style | ✓ |
| [MD036](rules/md036.md) | no-emphasis-as-heading | Emphasis used instead of a heading | ✓ |
| [MD037](rules/md037.md) | no-space-in-emphasis | Spaces inside emphasis markers | ✓ |
| [MD038](rules/md038.md) | no-space-in-code | Spaces inside code span elements | ✓ |
| [MD039](rules/md039.md) | no-space-in-links | Spaces inside link text | ✓ |
| [MD040](rules/md040.md) | fenced-code-language | Fenced code blocks should have a language specified | ✓ |
| [MD041](rules/md041.md) | first-line-heading | First line in a file should be a top-level heading | ✓ |
| [MD042](rules/md042.md) | no-empty-links | No empty links | Partial |
| [MD043](rules/md043.md) | required-headings | Required heading structure | ✗ |
| [MD044](rules/md044.md) | proper-names | Proper names should have the correct capitalization | ✓ |
| [MD045](rules/md045.md) | no-alt-text | Images should have alternate text (alt text) | ✓ |
| [MD046](rules/md046.md) | code-block-style | Code block style | ✓ |
| [MD047](rules/md047.md) | single-trailing-newline | Files should end with a single newline character | ✓ |
| [MD048](rules/md048.md) | code-fence-style | Code fence style | ✓ |
| [MD049](rules/md049.md) | emphasis-style | Emphasis style | ✓ |
| [MD050](rules/md050.md) | strong-style | Strong style | ✓ |
| [MD051](rules/md051.md) | link-fragments | Link fragments should be valid | ✗ |
| [MD052](rules/md052.md) | reference-links-images | Reference links and images should use a label that is defined | ✓ |
| [MD053](rules/md053.md) | link-image-reference-definitions | Link and image reference definitions should be needed | ✓ |
| [MD054](rules/md054.md) | link-image-style | Link and image style | Partial |
| [MD055](rules/md055.md) | table-pipe-style | Table pipe style | ✓ |
| [MD056](rules/md056.md) | table-column-count | Table column count | ✗ |
| [MD058](rules/md058.md) | blanks-around-tables | Tables should be surrounded by blank lines | ✓ |
| [MD059](rules/md059.md) | emphasis-markers | Emphasis marker style should not conflict with math syntax | ✓ |
| [MD060](rules/md060.md) | dollar-in-code-fence | Dollar signs in fenced code blocks | ✓ |

## Kramdown Extension Rules (KMD001-KMD011)

| Rule | Name | Description | Fixable |
|------|------|-------------|---------|
| [KMD001](rules/kmd001.md) | definition-list-term-has-definition | Definition list terms must have a definition | ✓ |
| [KMD002](rules/kmd002.md) | footnote-refs-defined | Footnote references must have matching definitions | ✓ |
| [KMD003](rules/kmd003.md) | footnote-defs-used | Footnote definitions must be referenced | ✓ |
| [KMD004](rules/kmd004.md) | abbreviation-defs-used | Abbreviation definitions should be used | ✓ |
| [KMD005](rules/kmd005.md) | no-duplicate-heading-ids | Heading IDs must be unique | ✓ |
| [KMD006](rules/kmd006.md) | valid-ial-syntax | IAL syntax must be well-formed | ✓ |
| [KMD007](rules/kmd007.md) | math-block-delimiters | Math block delimiters must be matched | ✓ |
| [KMD008](rules/kmd008.md) | block-extension-syntax | Block extensions must be properly opened and closed | Partial |
| [KMD009](rules/kmd009.md) | ald-defs-used | ALD definitions must be referenced | ✓ |
| [KMD010](rules/kmd010.md) | inline-ial-syntax | Inline IAL syntax must be well-formed | ✓ |
| [KMD011](rules/kmd011.md) | inline-math-balanced | Inline math spans must have balanced delimiters | ✗ |

## Legend

- ✓ = Auto-fixable with `--fix`
- ✗ = Not auto-fixable
- Partial = Partially auto-fixable (see rule doc for details)

## See Also

- [Upstream markdownlint documentation](https://github.com/DavidAnson/markdownlint/blob/main/doc/Rules.md)
- [Configuration guide](../README.md#configuration)
- [Contributing guide](../CONTRIBUTING.md)
