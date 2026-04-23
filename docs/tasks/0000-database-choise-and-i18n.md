Update all documentation and task lists to implement Multi-language support (RU/EN) and finalize the DB choice:

   1. Database: Standardize everything on PostgreSQL. Update docs/ARCHITECTURE.md, docs/DB_SCHEMA.md, and compose.yaml.
   2. Multi-language (i18n):
   * Choose the most suitable Rust libraries for i18n based on the current project structure.
      * Implement a strategy where SaaS mode handles dynamic user locales (stored in DB) and On-Premise can enforce a default locale via .env.
      * Move all hardcoded strings into external localization files.
   3. Documentation Sync:
   * Update docs/tasks/*.md to include database setup, migration handling, and i18n implementation steps.
      * Update AGENTS.md to ensure future AI tasks respect the Rust-based i18n and PostgreSQL architecture.
   4. Hybrid Model: Ensure the documentation clearly distinguishes how the Rust binary handles multi-tenancy in SaaS vs. single-tenancy in On-Premise.


Ensure all references to "SQLite" or "the database" are replaced with PostgreSQL. 

Output: Summary of modifications. Provide a brief Rust example showing how the chosen i18n library is integrated into a basic handler. 

