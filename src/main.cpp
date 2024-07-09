#include <filesystem>
#include <iostream>
#include <string>

#define ANSI_BOLD_TEXT "\x1b[1m"
#define ANSI_REGULAR_TEXT "\x1b[0m"

#define ANSI_COLOR_WHITE "\033[0m"
#define ANSI_COLOR_RED "\x1b[31m"
#define ANSI_COLOR_GREEN "\x1b[32m"
#define ANSI_COLOR_YELLOW "\x1b[33m"
#define ANSI_COLOR_BLUE "\x1b[34m"
#define ANSI_COLOR_MAGENTA "\x1b[35m"

bool ask_for_overwrite(const std::filesystem::path& path) {
    std::string response;
    std::cerr << ANSI_BOLD_TEXT << ANSI_COLOR_MAGENTA << "Warning" << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << ": " 
              << ANSI_BOLD_TEXT << ANSI_COLOR_GREEN << path.filename() << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE 
              << " already exists. Overwrite? (y/n): ";
    std::cin >> response;
    return (response == "y" || response == "Y");
}

void copy_file(const std::filesystem::path& source) {
    try {
        if (!std::filesystem::exists(source)) {
            std::cerr << ANSI_BOLD_TEXT << ANSI_COLOR_RED << "Error" << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << ": File "
                      << ANSI_BOLD_TEXT << ANSI_COLOR_GREEN << source << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << " does not exist.\n";
            return;
        }
        
        std::filesystem::path destination = std::filesystem::current_path() / source.filename();
        if (std::filesystem::exists(destination)) {
            if (!ask_for_overwrite(destination)) {
                std::cerr << ANSI_BOLD_TEXT << ANSI_COLOR_YELLOW << "Skipped" << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << ": File not overwritten.\n\t" 
                          << ANSI_BOLD_TEXT << ANSI_COLOR_BLUE << "~ Destination: " << ANSI_COLOR_YELLOW << destination << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << std::endl;
                return;
            }
        }
        std::filesystem::copy_file(source, destination, std::filesystem::copy_options::overwrite_existing);
        std::cerr << ANSI_BOLD_TEXT << ANSI_COLOR_GREEN << "Completed" << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << ": File copied to destination.\n\t" 
                  << ANSI_BOLD_TEXT << ANSI_COLOR_BLUE << "~ Destination: " << ANSI_COLOR_YELLOW << destination << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << std::endl;
    } catch (const std::filesystem::filesystem_error& e) {
        std::cerr << ANSI_BOLD_TEXT << ANSI_COLOR_MAGENTA << "Exception" << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << ":\n\t" 
                  << ANSI_BOLD_TEXT << ANSI_COLOR_BLUE << "~ e.what()" << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << ": " << ANSI_COLOR_YELLOW << "\"" << e.what() << "\"" << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << std::endl;
    }
}

void copy_directory(const std::filesystem::path& source) {
    try {
        if (!std::filesystem::exists(source) || !std::filesystem::is_directory(source)) {
            std::cerr << ANSI_BOLD_TEXT << ANSI_COLOR_RED << "Error" << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << ": Directory "
                      << ANSI_BOLD_TEXT << ANSI_COLOR_GREEN << source << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << " does not exist or is not a directory.\n";
            return;
        }

        std::filesystem::path destination = std::filesystem::current_path() / source.filename();
        if (std::filesystem::exists(destination)) {
            if (!ask_for_overwrite(destination)) {
                std::cerr << ANSI_BOLD_TEXT << ANSI_COLOR_YELLOW << "Skipped" << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << ": Directory not overwritten.\n\t" 
                          << ANSI_BOLD_TEXT << ANSI_COLOR_BLUE << "~ Destination: " << ANSI_COLOR_YELLOW << destination << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << std::endl;
                return;
            }
            std::filesystem::remove_all(destination);
        }
        std::filesystem::create_directory(destination);
        for (const auto& entry : std::filesystem::recursive_directory_iterator(source)) {
            const auto& path = entry.path();
            auto relative_path = std::filesystem::relative(path, source);
            std::filesystem::copy(path, destination / relative_path, std::filesystem::copy_options::overwrite_existing);
        }
        std::cerr << ANSI_BOLD_TEXT << ANSI_COLOR_GREEN << "Completed" << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << ": Directory copied to destination.\n\t" 
                  << ANSI_BOLD_TEXT << ANSI_COLOR_BLUE << "~ Destination: " << ANSI_COLOR_YELLOW << destination << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << std::endl;
    } catch (const std::filesystem::filesystem_error& e) {
        std::cerr << ANSI_BOLD_TEXT << ANSI_COLOR_MAGENTA << "Exception" << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << ":\n\t" 
                  << ANSI_BOLD_TEXT << ANSI_COLOR_BLUE << "~ e.what()" << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << ": " << ANSI_COLOR_YELLOW << "\"" << e.what() << "\"" << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << std::endl;
    }
}

int main(int argc, char **argv) {
    if (argc < 2) {
        std::cerr << ANSI_BOLD_TEXT << ANSI_COLOR_YELLOW << "Usage" << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << ": " << argv[0] << " <file|folder|dir> <path>" << std::endl;
        return 1;
    }

    if (argc < 3) {
        std::cerr << ANSI_BOLD_TEXT << ANSI_COLOR_RED << "Error" << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << ": Too few arguments.\n\t" 
                  << ANSI_BOLD_TEXT << ANSI_COLOR_BLUE << "~ First argument must be followed by a path." << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << std::endl;
        return 1;
    }

    std::string arg(argv[1]);
    if (arg == "file") {
        std::string file_path(argv[2]);
        copy_file(file_path);
    } else if (arg == "folder" || arg == "dir") {
        std::string dir_path(argv[2]);
        copy_directory(dir_path);
    } else {
        std::cerr << ANSI_BOLD_TEXT << ANSI_COLOR_RED << "Error" << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << ": Unknown first argument.\n\t" 
                  << ANSI_BOLD_TEXT << ANSI_COLOR_BLUE << "~ Argument: " << ANSI_COLOR_GREEN << arg << ANSI_REGULAR_TEXT << ANSI_COLOR_WHITE << std::endl;
    }
    return 0;
}
