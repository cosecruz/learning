# TC: Tools Compilation

> a CLI tools compilation- all in one cli app for dev tools
> Just a project to learn and master rust principles.

## **Tools**

- ### **arg_parser**

> ./tc parse --name=Alice --age=16 --verbose

#### **Conflicts**

- --verbose and --quiet
- --color and --no_color

#### **Explanation**

- flags are parsed and can be repeated but not conflict

```md
# Mind Maths

- ## What do i have?

  - flags; --name=Alice or --verbose
  - owned and gotten from args
  - parser function to parse flags
  - parsed: contains collection of flags

- ## What changes in transition?

  - Args: collection of raw flags: owns them
  - parser: takes ownership of flags -> checks_conflict(borrows)-> processes them into parsed(owns)

- ## When am i done

- all flags must be processed and parsed
- error on conflicts- cummulative error on all flags
- ## Invariant
- there must be no conflict
-
```

- ### **config_validator**

- ### **lazy_compute**

- ### **db_tx**
