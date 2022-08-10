use std::env;

use brainfuck_plus_interpreter::project_builder::Project;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_project = args[1].clone();

    let project = Project::init(path_project);
    project.build();
}