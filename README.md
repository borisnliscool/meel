![banner](https://github.com/user-attachments/assets/707f7cb0-f921-4f26-aff7-6efa68cc3d37)

### About the project

Meel is an open-source API for email scheduling and mailing list management, built with Rust. It features a complex templating system with layout files that can be edited to update all emails at once, streamlining the process of managing email designs. This system ensures consistency across email campaigns while reducing the time and effort required to make widespread changes.

<br/>

### Development notes

#### Creating new release

```
git tag -a <version> -m "v<version>" && git push --tags
```

#### Run Diesel migrations

```bash
diesel migration run
```

### RDD (Readme Driven Development)

- [x] Templating engine
    - [ ] Component system
    - [ ] .meel language syntax highlighting
    - [ ] i18n
    - [ ] simple if and for logic
- API Routes
    - [x] Sending mail
        - [x] Send to mailing list
        - [ ] File attachments
    - [x] Scheduling mail
    - [x] Fetching mail status
    - [x] Fetch templates list
    - [ ] Mailing lists
        - [x] Fetch lists
        - [x] Create mailing list
        - [x] Delete mailing lists
        - [ ] Update mailing lists name and description
        - [x] Add email to mailing list
        - [x] Remove from mailing list
- Configuration
    - [ ] Rate limiting
    - [ ] Mail server settings
    - [ ] Logging
    - [x] Maximum number of send attempts
    - [x] Template storage path

#### Error schema

```json
{
	"code": "<code>",
	"message": "<message>",
	"details": {
		"<key>": "<value>"
	}
}

```
