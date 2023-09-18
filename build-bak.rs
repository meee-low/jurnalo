// use std::process::Command;

// Commented out because it for some reason runs on *every* build, making them 5s long at minimum.

// fn main() {
//     println!("cargo:rerun-if-changed=src/backend/schema.rs");
//     println!("cargo:rerun-if-changed=migrations/");

//     const DIESEL_MIGRATION_RUN: &str =
//         "Failed to run migrations from diesel (`diesel migration run`)";
//     const DIESEL_MIGRATION_REDO: &str =
//         "Failed to redo migrations from diesel (`diesel migration redo`)";

//     let status = Command::new("diesel")
//         .arg("migration")
//         .arg("run")
//         .status()
//         .expect(DIESEL_MIGRATION_RUN);

//     if !status.success() {
//         panic!("{}", DIESEL_MIGRATION_RUN)
//     }

//     let status = Command::new("diesel")
//         .arg("migration")
//         .arg("redo")
//         .status()
//         .expect(DIESEL_MIGRATION_REDO);

//     if !status.success() {
//         panic!("{}", DIESEL_MIGRATION_REDO);
//     }
// }
