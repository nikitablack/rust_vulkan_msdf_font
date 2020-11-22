use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn is_input_file_outdated(input: &Path, output: &Path) -> io::Result<bool> {
    let out_meta = std::fs::metadata(output);
    if let Ok(meta) = out_meta {
        let output_mtime = meta.modified()?;

        // if input file is more recent than our output, we are outdated
        let input_meta = std::fs::metadata(input)?;
        let input_mtime = input_meta.modified()?;

        Ok(input_mtime > output_mtime)
    } else {
        // output file not found, we are outdated
        Ok(true)
    }
}

fn main() -> Result<(), i32> {
    let shaders = vec![
        "resources/vertex_shader.vert",
        "resources/fragment_shader.frag",
        "resources/fragment_shader_sdf.frag",
        "resources/fragment_shader_msdf.frag",
    ];

    for shader in shaders {
        let output_name = &format!("{}.spv", shader);

        match is_input_file_outdated(Path::new(shader), Path::new(output_name)) {
            Ok(false) => continue,
            _ => (),
        };

        let output = Command::new("glslangValidator")
            .args(&["-V", shader, "-o", &output_name])
            .output()
            .expect("failed to run glslangValidator");

        match output.status.code().unwrap_or(1) {
            0 => {
                io::stdout().write_all(&output.stdout).unwrap();
            }
            code => {
                io::stderr().write_all(&output.stdout).unwrap();
                return Err(code);
            }
        }
    }

    Ok(())
}
