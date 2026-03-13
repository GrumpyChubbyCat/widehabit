# imports.md

This rule governs how dependencies, structures, and functions are imported and referenced within the codebase. The goal is to minimize visual noise by avoiding fully qualified names (FQN) in the logic, while maintaining clarity during naming collisions.

## Guidelines

* **Prefer Direct Imports:** Always import structures, functions, and types directly at the top of the file. Avoid writing out the full path (e.g., `path::to::entity`) within the code body.
* **Explicit Scoping for Collisions:** If two entities from different modules share the same name, import the parent modules instead of the entities themselves. Use the module name as a prefix to disambiguate.
* *Example:* `import uuid;` and `import custom_uuid;` followed by `uuid::v4()` and `custom_uuid::v4()`.

* **Top-Level Organization:** Group imports logically (Standard Library, Third-party, Local) and ensure all external references are declared before the implementation begins.
* **Prohibit Inline Paths:** Do not use inline paths for types or functions unless it is strictly required to resolve a circular dependency that cannot be handled via standard imports.