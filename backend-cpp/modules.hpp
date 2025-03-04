#ifndef MODULES_HPP
#define MODULES_HPP

#include <string>
#include <vector>
#include <map>

// Represents a signal in a module.
struct Signal {
    std::string name;
    std::string direction; // "input", "output", "inout", or "internal"
    int width;
};

// Represents an instantiation of a submodule.
struct ModuleInstance {
    std::string instanceName;
    std::string moduleName;
    // Map from port name to connected net name.
    std::map<std::string, std::string> portConnections;
};

// Represents a module with its signals and submodule instantiations.
struct Module {
    std::string name;
    std::vector<Signal> signals;
    std::vector<ModuleInstance> submodules;
    int complexity;
};

#endif // MODULES_HPP
