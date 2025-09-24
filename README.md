todo add "Buy groceries" -d "Milk, eggs, bread" -p high --due 2025-09-20 --tags shopping,errands


🔹 1. Add a Task
Verb: add
Options:
-t, --title <string> → Task title (if not positional).
-d, --description <string> → Task description.
-p, --priority <low|medium|high> → Priority level.
--due <date> → Due date (YYYY-MM-DD).
--tags <tag1,tag2> → Comma-separated tags.
Arguments:
<title> (if used positionally)


`tool subcommand [options] [arguments]`

**Design Pattern:**
* **Subcommands** = verbs (what to do)
* **Options** = modifiers (how to do it)
* **Arguments** = objects (what to do it on)

🔹 **1. Subcommands**
Think of subcommands as verbs (actions) in a CLI.

* Examples:
  * `git commit -m "msg"`
  * `docker run -it ubuntu`

📌 **When to use subcommands:**
* If the tool has many distinct features (like Git, Docker, Kubernetes).
* If operations are clearly verbs (add, push, delete, list).
* If you want a namespace-like structure.


🔹 **2. Options (Flags)**
Options modify the behavior of the tool or subcommand.
* Short options: One letter, usually with `-`.
  * Example: `-v` (verbose), `-o output.txt`.
* Long options: Descriptive, start with --.
  * Example: `--verbose`, `--output=output.txt`.

📌 **Types of Options:**
1. Boolean switches – on/off flags  
    Example: `-v` or `--force`.
2. Key-value pairs – require a value  
    Example: `-o file.txt` or `--output=file.txt`.
3. Multiple occurrence – option can appear multiple times  
    Example: `-v -v -v` (increases verbosity).

📌 **Conventions:**
* `-h`, `--help` → Help
* `-V`, `--version` → Version info
* `-q`, `--quiet` → Suppress output
* `-v`, `--verbose` → Increase logging

// todo list -a
// todo list --all
// todo list --active
// todo list --completed

    // Mutually exclusive: --all | --active | --completed.

    // todo add "Buy groceries"
    // todo add "Buy groceries" --active
    // todo add "Buy groceries" --completed

    // todo edit 21 --t "Buy supplies"
    // todo edit 21 --title "Water the plants"
    // todo edit 21 --title "Pay bills" --active
    // todo edit 36 --active
    // todo edit 99 --completed
    // todo edit --all --active
    // todo edit --all --completed

    // todo delete 42
    // todo delete -a
    // todo delete --all
    // todo delete -a -f (skip confirmation)
    // todo delete --all --force
    // todo delete -completed
    // todo delete --completed --force

    // todo -h, --help
    // todo -v, --version
