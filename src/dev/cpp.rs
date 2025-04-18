use std::io::{self, Write};
use std::fs;
use colored::*;

fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

fn generate_gitignore() -> String {
    r#"# Compiled Object files
*.slo
*.lo
*.o
*.obj

# Precompiled Headers
*.gch
*.pch

# Compiled Dynamic libraries
*.so
*.dylib
*.dll

# Fortran module files
*.mod
*.smod

# Compiled Static libraries
*.lai
*.la
*.a
*.lib

# Executables
*.exe
*.out
*.app

# Build directories
build/
dist/
out/
bin/
obj/

# IDE specific files
.idea/
.vscode/
*.swp
*.swo
.DS_Store

# CMake files
CMakeFiles/
CMakeCache.txt
cmake_install.cmake
Makefile

# Environment variables
.env
.env.local

# Debug files
*.log
*.dSYM/

# Coverage files
coverage/
*.gcda
*.gcno

# Dependencies
vendor/
deps/

# Generated documentation
/doc/
/docs/

# Temporary files
*.tmp
*.temp
/tmp/
"#.to_string()
}

fn generate_readme(project_name: &str, template: &str) -> String {
    match template {
        "standard" => format!(r#"# {} - Standard C++ Project

A modern C++ project with CMake and Makefile support.

## Getting Started

### Prerequisites

- C++ compiler (g++ or clang++)
- Make
- CMake (version 3.10 or higher)

### Installation

```bash
git clone <repository-url>
cd {}
mkdir build && cd build
cmake ..
make
```

### Running Tests

```bash
cd build
ctest
```

## Project Structure

```
{}
├── src/
│   ├── main.cpp
│   └── include/
│       └── {}.hpp
├── tests/
│   └── main_test.cpp
├── CMakeLists.txt
├── Makefile
└── README.md
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
"#, project_name, project_name, project_name, project_name),
        "qt" => format!(r#"# {} - Qt Application

A C++ GUI application built with Qt framework.

## Getting Started

### Prerequisites

- C++ compiler (g++ or clang++)
- Make
- CMake (version 3.10 or higher)
- Qt (version 6 or higher)
- Qt Creator (optional, for development)

### Installation

```bash
git clone <repository-url>
cd {}
mkdir build && cd build
cmake ..
make
```

### Running Tests

```bash
cd build
ctest
```

## Project Structure

```
{}
├── src/
│   ├── main.cpp
│   ├── mainwindow.cpp
│   ├── mainwindow.h
│   └── include/
│       └── {}.hpp
├── tests/
│   └── main_test.cpp
├── CMakeLists.txt
├── Makefile
└── README.md
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
"#, project_name, project_name, project_name, project_name),
        "opengl" => format!(r#"# {} - OpenGL Application

A C++ 3D graphics application built with OpenGL.

## Getting Started

### Prerequisites

- C++ compiler (g++ or clang++)
- Make
- CMake (version 3.10 or higher)
- OpenGL (version 3.3 or higher)
- GLFW
- GLAD
- GLM

### Installation

```bash
git clone <repository-url>
cd {}
mkdir build && cd build
cmake ..
make
```

### Running Tests

```bash
cd build
ctest
```

## Project Structure

```
{}
├── src/
│   ├── main.cpp
│   ├── shader.cpp
│   ├── shader.h
│   └── include/
│       └── {}.hpp
├── tests/
│   └── main_test.cpp
├── CMakeLists.txt
├── Makefile
└── README.md
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
"#, project_name, project_name, project_name, project_name),
        _ => format!(r#"# {}

A C++ project created with Gremlin CLI.

## Getting Started

### Prerequisites

- C++ compiler (g++ or clang++)
- Make
- CMake (version 3.10 or higher)

### Installation

```bash
git clone <repository-url>
cd {}
mkdir build && cd build
cmake ..
make
```

### Running Tests

```bash
cd build
ctest
```

## Project Structure

```
{}
├── src/
│   ├── main.cpp
│   └── include/
│       └── {}.hpp
├── tests/
│   └── main_test.cpp
├── CMakeLists.txt
├── Makefile
└── README.md
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
"#, project_name, project_name, project_name, project_name)
    }
}

fn generate_cmakelists(project_name: &str, template: &str) -> String {
    match template {
        "standard" => format!(r#"cmake_minimum_required(VERSION 3.10)
project({})

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Add source files
file(GLOB SOURCES "src/*.cpp")
file(GLOB HEADERS "include/*.hpp")

# Create executable
add_executable(${{PROJECT_NAME}} ${{SOURCES}} ${{HEADERS}})

# Include directories
target_include_directories(${{PROJECT_NAME}} PRIVATE include)

# Add tests if needed
enable_testing()
add_subdirectory(tests)
"#, project_name),
        "qt" => format!(r#"cmake_minimum_required(VERSION 3.10)
project({})

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)

find_package(Qt6 REQUIRED COMPONENTS Core Gui Widgets)

# Add source files
file(GLOB SOURCES "src/*.cpp")
file(GLOB HEADERS "include/*.hpp")
file(GLOB RESOURCES "resources/*.qrc")

# Create executable
add_executable(${{PROJECT_NAME}} ${{SOURCES}} ${{HEADERS}} ${{RESOURCES}})

# Include directories
target_include_directories(${{PROJECT_NAME}} PRIVATE include)

# Link Qt libraries
target_link_libraries(${{PROJECT_NAME}} PRIVATE
    Qt6::Core
    Qt6::Gui
    Qt6::Widgets
)
"#, project_name),
        "opengl" => format!(r#"cmake_minimum_required(VERSION 3.10)
project({})

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find required packages
find_package(OpenGL REQUIRED)
find_package(glfw3 REQUIRED)
find_package(GLM REQUIRED)

# Add source files
file(GLOB SOURCES "src/*.cpp")
file(GLOB HEADERS "include/*.hpp")

# Create executable
add_executable(${{PROJECT_NAME}} ${{SOURCES}} ${{HEADERS}})

# Include directories
target_include_directories(${{PROJECT_NAME}} PRIVATE include)

# Link libraries
target_link_libraries(${{PROJECT_NAME}} PRIVATE
    OpenGL::GL
    glfw
    glm
)
"#, project_name),
        _ => format!(r#"cmake_minimum_required(VERSION 3.10)
project({})

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Add source files
file(GLOB SOURCES "src/*.cpp")
file(GLOB HEADERS "include/*.hpp")

# Create executable
add_executable(${{PROJECT_NAME}} ${{SOURCES}} ${{HEADERS}})

# Include directories
target_include_directories(${{PROJECT_NAME}} PRIVATE include)

# Add tests if needed
enable_testing()
add_subdirectory(tests)
"#, project_name)
    }
}

fn generate_makefile(project_name: &str, template: &str) -> String {
    match template {
        "standard" => format!(r#"CXX = g++
CXXFLAGS = -std=c++17 -Wall -Wextra -Iinclude
LDFLAGS = 

SRC_DIR = src
OBJ_DIR = obj
BIN_DIR = bin

SRCS = $(wildcard $(SRC_DIR)/*.cpp)
OBJS = $(SRCS:$(SRC_DIR)/%.cpp=$(OBJ_DIR)/%.o)
TARGET = $(BIN_DIR)/{}

.PHONY: all clean

all: $(TARGET)

$(TARGET): $(OBJS)
	@mkdir -p $(BIN_DIR)
	$(CXX) $(OBJS) -o $@ $(LDFLAGS)

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.cpp
	@mkdir -p $(OBJ_DIR)
	$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	rm -rf $(OBJ_DIR) $(BIN_DIR)
"#, project_name),
        "qt" => format!(r#"CXX = g++
CXXFLAGS = -std=c++17 -Wall -Wextra -Iinclude $(shell pkg-config --cflags Qt6Core Qt6Gui Qt6Widgets)
LDFLAGS = $(shell pkg-config --libs Qt6Core Qt6Gui Qt6Widgets)

SRC_DIR = src
OBJ_DIR = obj
BIN_DIR = bin

SRCS = $(wildcard $(SRC_DIR)/*.cpp)
OBJS = $(SRCS:$(SRC_DIR)/%.cpp=$(OBJ_DIR)/%.o)
TARGET = $(BIN_DIR)/{}

.PHONY: all clean

all: $(TARGET)

$(TARGET): $(OBJS)
	@mkdir -p $(BIN_DIR)
	$(CXX) $(OBJS) -o $@ $(LDFLAGS)

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.cpp
	@mkdir -p $(OBJ_DIR)
	$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	rm -rf $(OBJ_DIR) $(BIN_DIR)
"#, project_name),
        "opengl" => format!(r#"CXX = g++
CXXFLAGS = -std=c++17 -Wall -Wextra -Iinclude $(shell pkg-config --cflags gl glfw3 glm)
LDFLAGS = $(shell pkg-config --libs gl glfw3 glm)

SRC_DIR = src
OBJ_DIR = obj
BIN_DIR = bin

SRCS = $(wildcard $(SRC_DIR)/*.cpp)
OBJS = $(SRCS:$(SRC_DIR)/%.cpp=$(OBJ_DIR)/%.o)
TARGET = $(BIN_DIR)/{}

.PHONY: all clean

all: $(TARGET)

$(TARGET): $(OBJS)
	@mkdir -p $(BIN_DIR)
	$(CXX) $(OBJS) -o $@ $(LDFLAGS)

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.cpp
	@mkdir -p $(OBJ_DIR)
	$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	rm -rf $(OBJ_DIR) $(BIN_DIR)
"#, project_name),
        _ => format!(r#"CXX = g++
CXXFLAGS = -std=c++17 -Wall -Wextra -Iinclude
LDFLAGS = 

SRC_DIR = src
OBJ_DIR = obj
BIN_DIR = bin

SRCS = $(wildcard $(SRC_DIR)/*.cpp)
OBJS = $(SRCS:$(SRC_DIR)/%.cpp=$(OBJ_DIR)/%.o)
TARGET = $(BIN_DIR)/{}

.PHONY: all clean

all: $(TARGET)

$(TARGET): $(OBJS)
	@mkdir -p $(BIN_DIR)
	$(CXX) $(OBJS) -o $@ $(LDFLAGS)

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.cpp
	@mkdir -p $(OBJ_DIR)
	$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	rm -rf $(OBJ_DIR) $(BIN_DIR)
"#, project_name)
    }
}

fn create_standard_project(project_name: &str) {
    let base_path = "/home/klea/Documents/Dev";
    let project_path = format!("{}/{}", base_path, project_name);
    
    println!("{}", format!("Creating standard C++ project '{}'...", project_name).bright_cyan());
    
    // Create project directory structure
    let dirs = vec![
        format!("{}/src", project_path),
        format!("{}/include", project_path),
        format!("{}/tests", project_path),
    ];
    
    for dir in dirs {
        if let Err(e) = std::fs::create_dir_all(&dir) {
            println!("{} {}", "Failed to create directory:".bright_red(), e);
            return;
        }
    }
    
    // Create main.cpp
    let main_cpp = r#"#include <iostream>
#include "project_name.hpp"

int main() {
    std::cout << "Hello, World!" << std::endl;
    return 0;
}"#.replace("project_name", project_name);
    
    if let Err(e) = std::fs::write(format!("{}/src/main.cpp", project_path), main_cpp) {
        println!("{} {}", "Failed to create main.cpp:".bright_red(), e);
        return;
    }
    
    // Create header file
    let header_hpp = r#"#pragma once

#include <string>

namespace project_name {
    class Project {
    public:
        Project();
        ~Project();
        
        void run();
    private:
        std::string name;
    };
}"#.replace("project_name", project_name);
    
    if let Err(e) = std::fs::write(format!("{}/include/{}.hpp", project_path, project_name), header_hpp) {
        println!("{} {}", "Failed to create header file:".bright_red(), e);
        return;
    }
    
    // Create CMakeLists.txt
    if let Err(e) = std::fs::write(format!("{}/CMakeLists.txt", project_path), generate_cmakelists(project_name, "standard")) {
        println!("{} {}", "Failed to create CMakeLists.txt:".bright_red(), e);
        return;
    }
    
    // Create Makefile
    if let Err(e) = std::fs::write(format!("{}/Makefile", project_path), generate_makefile(project_name, "standard")) {
        println!("{} {}", "Failed to create Makefile:".bright_red(), e);
        return;
    }
    
    // Create README.md
    if let Err(e) = std::fs::write(format!("{}/README.md", project_path), generate_readme(project_name, "standard")) {
        println!("{} {}", "Failed to create README.md:".bright_red(), e);
        return;
    }
    
    // Create .gitignore
    if let Err(e) = std::fs::write(format!("{}/.gitignore", project_path), generate_gitignore()) {
        println!("{} {}", "Failed to create .gitignore:".bright_red(), e);
        return;
    }
    
    println!("{}", "✅ Standard C++ project created successfully!".bright_green());
    println!("{}", format!("Project location: {}", project_path).bright_cyan());
    println!("\n{}", "Project structure:".bright_yellow());
    display_file_structure(&project_path, "");
}

fn create_qt_project(project_name: &str) {
    let base_path = "/home/klea/Documents/Dev";
    let project_path = format!("{}/{}", base_path, project_name);
    
    println!("{}", format!("Creating Qt project '{}'...", project_name).bright_cyan());
    
    // Create project directory structure
    let dirs = vec![
        format!("{}/src", project_path),
        format!("{}/include", project_path),
        format!("{}/resources", project_path),
        format!("{}/resources/icons", project_path),
    ];
    
    for dir in dirs {
        if let Err(e) = std::fs::create_dir_all(&dir) {
            println!("{} {}", "Failed to create directory:".bright_red(), e);
            return;
        }
    }
    
    // Create main.cpp
    let main_cpp = r#"#include <QApplication>
#include "mainwindow.hpp"

int main(int argc, char *argv[]) {
    QApplication app(argc, argv);
    
    MainWindow window;
    window.show();
    
    return app.exec();
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/src/main.cpp", project_path), main_cpp) {
        println!("{} {}", "Failed to create main.cpp:".bright_red(), e);
        return;
    }
    
    // Create mainwindow.hpp
    let mainwindow_hpp = r#"#pragma once

#include <QMainWindow>
#include <QWidget>

class MainWindow : public QMainWindow {
    Q_OBJECT
    
public:
    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();
    
private:
    void setupUi();
    void setupConnections();
};"#;
    
    if let Err(e) = std::fs::write(format!("{}/include/mainwindow.hpp", project_path), mainwindow_hpp) {
        println!("{} {}", "Failed to create mainwindow.hpp:".bright_red(), e);
        return;
    }
    
    // Create mainwindow.cpp
    let mainwindow_cpp = r#"#include "mainwindow.hpp"
#include <QPushButton>
#include <QVBoxLayout>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent) {
    setupUi();
    setupConnections();
}

MainWindow::~MainWindow() {}

void MainWindow::setupUi() {
    auto centralWidget = new QWidget(this);
    setCentralWidget(centralWidget);
    
    auto layout = new QVBoxLayout(centralWidget);
    
    auto button = new QPushButton("Click me!", centralWidget);
    layout->addWidget(button);
}

void MainWindow::setupConnections() {
    // Add your signal-slot connections here
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/src/mainwindow.cpp", project_path), mainwindow_cpp) {
        println!("{} {}", "Failed to create mainwindow.cpp:".bright_red(), e);
        return;
    }
    
    // Create resources.qrc
    let resources_qrc = r#"<!DOCTYPE RCC>
<RCC version="1.0">
    <qresource prefix="/">
        <!-- Add your resources here -->
    </qresource>
</RCC>"#;
    
    if let Err(e) = std::fs::write(format!("{}/resources/resources.qrc", project_path), resources_qrc) {
        println!("{} {}", "Failed to create resources.qrc:".bright_red(), e);
        return;
    }
    
    // Create CMakeLists.txt
    if let Err(e) = std::fs::write(format!("{}/CMakeLists.txt", project_path), generate_cmakelists(project_name, "qt")) {
        println!("{} {}", "Failed to create CMakeLists.txt:".bright_red(), e);
        return;
    }
    
    // Create Makefile
    if let Err(e) = std::fs::write(format!("{}/Makefile", project_path), generate_makefile(project_name, "qt")) {
        println!("{} {}", "Failed to create Makefile:".bright_red(), e);
        return;
    }
    
    // Create README.md
    if let Err(e) = std::fs::write(format!("{}/README.md", project_path), generate_readme(project_name, "qt")) {
        println!("{} {}", "Failed to create README.md:".bright_red(), e);
        return;
    }
    
    // Create .gitignore
    if let Err(e) = std::fs::write(format!("{}/.gitignore", project_path), generate_gitignore()) {
        println!("{} {}", "Failed to create .gitignore:".bright_red(), e);
        return;
    }
    
    println!("{}", "✅ Qt project created successfully!".bright_green());
    println!("{}", format!("Project location: {}", project_path).bright_cyan());
    println!("\n{}", "Project structure:".bright_yellow());
    display_file_structure(&project_path, "");
}

fn create_opengl_project(project_name: &str) {
    let base_path = "/home/klea/Documents/Dev";
    let project_path = format!("{}/{}", base_path, project_name);
    
    println!("{}", format!("Creating OpenGL project '{}'...", project_name).bright_cyan());
    
    // Create project directory structure
    let dirs = vec![
        format!("{}/src", project_path),
        format!("{}/include", project_path),
        format!("{}/resources", project_path),
        format!("{}/resources/shaders", project_path),
        format!("{}/resources/models", project_path),
    ];
    
    for dir in dirs {
        if let Err(e) = std::fs::create_dir_all(&dir) {
            println!("{} {}", "Failed to create directory:".bright_red(), e);
            return;
        }
    }
    
    // Create main.cpp
    let main_cpp = r#"#include <GLFW/glfw3.h>
#include <iostream>
#include "shader.hpp"
#include "mesh.hpp"

int main() {
    // Initialize GLFW
    if (!glfwInit()) {
        std::cerr << "Failed to initialize GLFW" << std::endl;
        return -1;
    }
    
    // Create window
    GLFWwindow* window = glfwCreateWindow(800, 600, "OpenGL Project", nullptr, nullptr);
    if (!window) {
        std::cerr << "Failed to create GLFW window" << std::endl;
        glfwTerminate();
        return -1;
    }
    
    // Make the window's context current
    glfwMakeContextCurrent(window);
    
    // Main loop
    while (!glfwWindowShouldClose(window)) {
        // Clear the screen
        glClear(GL_COLOR_BUFFER_BIT);
        
        // Render here
        
        // Swap front and back buffers
        glfwSwapBuffers(window);
        
        // Poll for and process events
        glfwPollEvents();
    }
    
    // Clean up
    glfwTerminate();
    return 0;
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/src/main.cpp", project_path), main_cpp) {
        println!("{} {}", "Failed to create main.cpp:".bright_red(), e);
        return;
    }
    
    // Create shader.hpp
    let shader_hpp = r#"#pragma once

#include <string>
#include <GL/glew.h>

class Shader {
public:
    Shader(const std::string& vertexPath, const std::string& fragmentPath);
    ~Shader();
    
    void use();
    void setUniform(const std::string& name, float value);
    
private:
    GLuint program;
    
    GLuint compileShader(GLenum type, const std::string& source);
    void linkProgram();
};"#;
    
    if let Err(e) = std::fs::write(format!("{}/include/shader.hpp", project_path), shader_hpp) {
        println!("{} {}", "Failed to create shader.hpp:".bright_red(), e);
        return;
    }
    
    // Create mesh.hpp
    let mesh_hpp = r#"#pragma once

#include <vector>
#include <GL/glew.h>
#include <glm/glm.hpp>

struct Vertex {
    glm::vec3 position;
    glm::vec3 normal;
    glm::vec2 texCoords;
};

class Mesh {
public:
    Mesh(const std::vector<Vertex>& vertices, const std::vector<unsigned int>& indices);
    ~Mesh();
    
    void draw();
    
private:
    GLuint VAO, VBO, EBO;
    std::vector<Vertex> vertices;
    std::vector<unsigned int> indices;
    
    void setupMesh();
};"#;
    
    if let Err(e) = std::fs::write(format!("{}/include/mesh.hpp", project_path), mesh_hpp) {
        println!("{} {}", "Failed to create mesh.hpp:".bright_red(), e);
        return;
    }
    
    // Create CMakeLists.txt
    if let Err(e) = std::fs::write(format!("{}/CMakeLists.txt", project_path), generate_cmakelists(project_name, "opengl")) {
        println!("{} {}", "Failed to create CMakeLists.txt:".bright_red(), e);
        return;
    }
    
    // Create Makefile
    if let Err(e) = std::fs::write(format!("{}/Makefile", project_path), generate_makefile(project_name, "opengl")) {
        println!("{} {}", "Failed to create Makefile:".bright_red(), e);
        return;
    }
    
    // Create README.md
    if let Err(e) = std::fs::write(format!("{}/README.md", project_path), generate_readme(project_name, "opengl")) {
        println!("{} {}", "Failed to create README.md:".bright_red(), e);
        return;
    }
    
    // Create .gitignore
    if let Err(e) = std::fs::write(format!("{}/.gitignore", project_path), generate_gitignore()) {
        println!("{} {}", "Failed to create .gitignore:".bright_red(), e);
        return;
    }
    
    println!("{}", "✅ OpenGL project created successfully!".bright_green());
    println!("{}", format!("Project location: {}", project_path).bright_cyan());
    println!("\n{}", "Project structure:".bright_yellow());
    display_file_structure(&project_path, "");
}

fn display_file_structure(path: &str, prefix: &str) {
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                
                if name_str.starts_with('.') {
                    continue;
                }
                
                println!("{}{}", prefix, name_str.bright_cyan());
                
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    display_file_structure(&entry.path().to_string_lossy(), &format!("{}  ", prefix));
                }
            }
        }
    }
}

fn create_new_project() {
    println!("\n{}", "C++ Project Templates:".bright_cyan());
    println!("{} {}", "1.".bright_green(), "Standard C++ Project 🏗️".bright_yellow());
    println!("{} {}", "2.".bright_green(), "Qt GUI Project 🖥️".bright_magenta());
    println!("{} {}", "3.".bright_green(), "OpenGL 3D Project 🎮".bright_blue());
    println!("{} {}", "4.".bright_green(), "Back to C++ Dashboard 🔙".bright_blue());

    print!("\n{}", "Select template (1-4): ".bright_blue());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    match input.trim() {
        "1" => {
            print!("{}", "Enter project name: ".bright_blue());
            io::stdout().flush().unwrap();
            let mut name = String::new();
            io::stdin().read_line(&mut name).unwrap();
            create_standard_project(name.trim());
        },
        "2" => {
            print!("{}", "Enter project name: ".bright_blue());
            io::stdout().flush().unwrap();
            let mut name = String::new();
            io::stdin().read_line(&mut name).unwrap();
            create_qt_project(name.trim());
        },
        "3" => {
            print!("{}", "Enter project name: ".bright_blue());
            io::stdout().flush().unwrap();
            let mut name = String::new();
            io::stdin().read_line(&mut name).unwrap();
            create_opengl_project(name.trim());
        },
        "4" => return,
        _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
    }
}

pub fn run() {
    let splash_art = load_ascii("ascii/CppGirl.txt");
    println!("{}", splash_art.bright_yellow());
    println!("{}", "🏗️ C++ Project Dashboard 🏗️".bright_yellow().bold().blink());
    
    loop {
        println!("\n{}", "C++ Project Menu:".bright_cyan());
        println!("{} {}", "1.".bright_green(), "Create New Project 🏗️".bright_yellow());
        println!("{} {}", "2.".bright_green(), "Return to Project Builder 🔙".bright_blue());

        print!("\n{}", "Enter your choice (1-2): ".bright_blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => create_new_project(),
            "2" => {
                println!("{}", "Returning to Project Builder...".bright_blue());
                return;
            },
            _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
        }
    }
}