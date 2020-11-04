# About
Pickem is a tool to select a value from a tree.
The tree is specified in an yml file in a declarative manner.
One example would be to buld a tree of shortcuts to run a command.

This is a generic tool and the goal is to provide different interfaces for it.

# Yaml Spec
Each entry in the yaml file represents a node.
At a high level, each node can be thought of as a tuple with 5 values:
- Name: Name is displayed next to the cord, specifies a short human description of the node. 
- Desc: Verbose description of what a node represents.
- Chord: character or string of characters used to activate node (eg. `ab`, `e`).
- Nodes: List of children nodes
- Value: The value stored by a node. In the absence of a value, name will be returned.

The string used as the yml key implicitly defines the name for that node.
Asside from the reserved keywords, any subvalue of a yaml's entry will be considered a child.
Save from the reserved keywords, any value can be used to specify a name.

The following keywords are reserved:
- `.value` -> Specifies the return value for the selection. `value` must be specified only in leaves as it is a shortcircuiting keyword.
- `.chord` -> Indicates which character(s) are used to activate the node.
- `.desc` -> Verbose description of what the node represents.

Example
```yaml
programs:
  .chord: p
  .desc: shortcuts to open common programs
  chromium:
    .chord: c
    .value: chromium
  discord:
    .chord: d
    .value: discord
  zathura:
    .chord: z
    .value: zathura
  flameshot:
    .chord: f
    full:
      .chord: a
      .value: flameshot full
    gui:
      .chord: g
      .value: flameshot gui
```
