<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/c315ec64-9b66-4e75-955e-034a542def11">
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/707f7cb0-f921-4f26-aff7-6efa68cc3d37">
  <img alt="Shows the black Meel logo in light theme, and the white logo in dark theme." src="https://github.com/user-attachments/assets/707f7cb0-f921-4f26-aff7-6efa68cc3d37">
</picture>

### About the project

Meel is an open-source API for email scheduling and mailing list management, built with Rust. It features a complex
templating system with layout files that can be edited to update all emails at once, streamlining the process of
managing email designs. This system ensures consistency across email campaigns while reducing the time and effort
required to make widespread changes.

### Usage

To use Meel, you can either compile the backend yourself, or use the automatically built Docker image. The Docker image
is available on GitHub at [borisnliscool/meel](https://github.com/borisnliscool/meel/pkgs/container/meel).

**Example**

```yml
services:
  db:
    image: postgres
    restart: unless-stopped
    shm_size: 128mb
    env_file:
      - .env.production
    volumes:
      - db_data:/var/lib/postgresql/data

  meel:
    image: ghcr.io/borisnliscool/meel:latest
    restart: unless-stopped
    ports:
      - "8080:8080"
    env_file:
      - .env.production
    depends_on:
      - db
    volumes:
      - ./data:/usr/src/meel/data

volumes:
  db_data:
```

### Repository overview

This monorepo includes the main API, SDKs for supported languages, and tools to simplify the use of Meel templating syntax. 
Refer to the individual SDK directories for detailed documentation.

| Directory                                     | Description                            |
|-----------------------------------------------|----------------------------------------|
| [backend](./backend)                          | Core API functionality                 |
| [sdk/node](./sdk/node)                        | Node SDK for the API                   |

<br/>

### Development notes

#### Creating new release

```
git tag -a <version> -m "v<version>" && git push --tags
```

#### Run Diesel migrations

To create the initial database schema and relations, run the following command
(you may need to install the diesel cli first by running `cargo install diesel_cli`):

```bash
cd backend
diesel migration run
```

### RDD (Readme Driven Development)

- [x] Templating engine
    - [ ] Component system
    - [ ] i18n
    - [ ] simple if and for logic
- API Routes
    - [x] Sending mail
        - [x] Send to mailing list
        - [ ] File attachments
        - [ ] Validate email sender and recipient names
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
    - [x] Mail server settings
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
