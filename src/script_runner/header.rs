use chrono::Local;

#[allow(dead_code)]
pub fn generate_header(filename: &str, script_type: &str) -> String {
    let date = Local::now().format("%Y-%m-%d").to_string();
    
    let header_template = match script_type {
        "Python" => format!(r#"#!/usr/bin/env python3
# -*- coding: utf-8 -*-

# 📄  File        : {}
# 🗓️   Created     : {}
# 🕒  Last Update : {} 
# 🧬  Version     : 0.1.0
# ✍️   Author      : KleaSCM — https://github.com/KleaSCM
# 📝  Description : <Short summary of what this script does>
# 🔒  License     : MIT
# 💡  Usage       : python3 {}
# 📦  Dependencies: <list of Python packages>

"#, filename, date, date, filename),
        "Rust" => format!(r#"// 📄  File        : {}
// 🗓️   Created     : {}
// 🕒  Last Update : {} 
// 🧬  Version     : 0.1.0
// ✍️   Author      : KleaSCM — https://github.com/KleaSCM
// 📝  Description : <Short summary of what this script does>
// 🔒  License     : MIT
// 💡  Usage       : cargo run
// 📦  Dependencies: <list of Rust crates>

"#, filename, date, date),
        "Bash" => format!(r#"#!/bin/bash

# 📄  File        : {}
# 🗓️   Created     : {}
# 🕒  Last Update : {} 
# 🧬  Version     : 0.1.0
# ✍️   Author      : KleaSCM — https://github.com/KleaSCM
# 📝  Description : <Short summary of what this script does>
# 🔒  License     : MIT
# 💡  Usage       : bash {}
# 📦  Dependencies: <list of CLI tools>

"#, filename, date, date, filename),
        "Go" => format!(r#"// 📄  File        : {}
// 🗓️   Created     : {}
// 🕒  Last Update : {} 
// 🧬  Version     : 0.1.0
// ✍️   Author      : KleaSCM — https://github.com/KleaSCM
// 📝  Description : <Short summary of what this script does>
// 🔒  License     : MIT
// 💡  Usage       : go run {}
// 📦  Dependencies: <list of Go packages>

"#, filename, date, date, filename),
        "Lua" => format!(r#"#!/usr/bin/env lua

-- 📄  File        : {}
-- 🗓️   Created     : {}
-- 🕒  Last Update : {} 
-- 🧬  Version     : 0.1.0
-- ✍️   Author      : KleaSCM — https://github.com/KleaSCM
-- 📝  Description : <Short summary of what this script does>
-- 🔒  License     : MIT
-- 💡  Usage       : lua {}
-- 📦  Dependencies: <list of Lua modules>

"#, filename, date, date, filename),
        "PS1" => format!(r#"<#
    📄  File        : {}
    🗓️   Created     : {}
    🕒  Last Update : {} 
    🧬  Version     : 0.1.0
    ✍️   Author      : KleaSCM — https://github.com/KleaSCM
    📝  Description : <Short summary of what this script does>
    🔒  License     : MIT
    💡  Usage       : pwsh {}
    📦  Dependencies: <list of PowerShell modules>
#>

"#, filename, date, date, filename),
        _ => unreachable!(),
    };

    header_template
} 