use std::io;

pub struct AnispeakOptions {
    pub message: String,
    pub animal: String,
    pub width: usize,
}

pub fn run_anispeak(options: AnispeakOptions) -> io::Result<()> {
    let bubble = get_bubble(&options.message, options.width);
    let animal_art = get_animal_ascii(&options.animal.to_lowercase());
    print!("{}{}", bubble, animal_art);
    println!();
    Ok(())
}

fn get_bubble(text: &str, max_width: usize) -> String {
    let lines = wrap_text(text, max_width);
    let max_line_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    
    let mut output = String::new();
    // Top border
    output.push_str("  ");
    for _ in 0..max_line_len + 2 {
        output.push('_');
    }
    output.push('\n');

    if lines.len() == 1 {
        output.push_str(&format!("< {} >\n", lines[0]));
    } else {
        for (i, line) in lines.iter().enumerate() {
            let padding = " ".repeat(max_line_len - line.len());
            if i == 0 {
                output.push_str(&format!("/ {}{} \\\n", line, padding));
            } else if i == lines.len() - 1 {
                output.push_str(&format!("\\ {}{} /\n", line, padding));
            } else {
                output.push_str(&format!("| {}{} |\n", line, padding));
            }
        }
    }

    // Bottom border
    output.push_str("  ");
    for _ in 0..max_line_len + 2 {
        output.push('-');
    }
    output.push('\n');
    output
}

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.lines() {
        if paragraph.trim().is_empty() {
            lines.push(String::new());
            continue;
        }
        let mut current_line = String::new();
        for word in paragraph.split_whitespace() {
            if current_line.is_empty() {
                current_line.push_str(word);
            } else if current_line.len() + 1 + word.len() <= max_width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                lines.push(current_line);
                current_line = word.to_string();
            }
        }
        if !current_line.is_empty() {
            lines.push(current_line);
        }
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

fn get_animal_ascii(animal: &str) -> &'static str {
    match animal {
        "crab" => {
            r#"        \
         \   _~^~^~_
          \ (\'o o'/)
            '_\|_|/'_
              /   \"#
        }
        "dino" => {
            r#"        \
         \    __
             / _)
      .-^^^-/ /
     __/       /
    <__.|_|-|_|"#
        }
        "cat" => {
            r#"        \
         \  /\_/\  
           ( o.o ) 
            > ^ <  "#
        }
        "dog" => {
            r#"        \
         \   /^ ^\
            / 0 0 \
            V\ Y /V
             / - \
            /    |"#
        }
        "duck" => {
            r#"        \
         \   _
            (o> 
         _(\  )
        (___(_)_"#
        }
        "owl" => {
            r#"        \
         \  {o,o}
            /) )
            -"-"-"#
        }
        "penguin" => {
            r#"        \
         \   (o_o)
            /(   )\
            ^^" "^^"#
        }
        "elephant" => {
            r#"        \
         \    _.-.
          \.-'    `.__
         (o        `  `._
         /               `.
         | \             | \
         |  | /)_____|)  |  |
         |  | |      |   |  |     "#
        }
        "moose" => {
            r#"        \
         \  \_\_    _/_/
             \__\__/__/
              (oo)\_______
              (__)\       )\/\
                  ||----w |
                  ||     ||     "#
        }
        "stegosaurus" => {
            r#"        \
         \    /\  /\  /\
            _/  \/  \/  \_
          /               \__
         |  (o)           |  \
         \               / __/
          \_/|_|\_/|_|\_/     "#
        }
        "whale" => {
            r#"        \
         \       .
                .
                 .
              _.-'''`-,_
            .'          `._
           /               `-.
          |  o            _.-'
          \          _.-'
           `._.---''          "#
        }
        "snake" => {
            r#"        \
         \    _
             /  \
            / o  \
           |   __/
            \  \
            /  /
          _/  /_
         (______)             "#
        }
        "turtle" => {
            r#"        \
         \     __
           ___(_oo)
          (______)
          //    \\"#
        }
        "sheep" => {
            r#"        \
         \   __      
            (oo)     
    /\  ____(__)     
   /  \/     ||      
  *   ||-----||      
      ~~     ~~      "#
        }
        "cow" | _ => {
            r#"        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||"#
        }
    }
}
