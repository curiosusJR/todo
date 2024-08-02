# todo

[![CI](https://github.com/thekuwayama/todo/workflows/CI/badge.svg)](https://github.com/thekuwayama/todo/actions?workflow=CI)
[![MIT licensed](https://img.shields.io/badge/license-MIT-brightgreen.svg)](https://raw.githubusercontent.com/thekuwayama/todo/master/LICENSE.txt)
[![dependency status](https://deps.rs/repo/github/thekuwayama/todo/status.svg)](https://deps.rs/repo/github/thekuwayama/todo)

`todo` is a simple todo list command-line tool written in Rust.
forked from <https://github.com/thekuwayama/todo.git>

## Install

You can install `todo` with the following:

```sh-session
$ cargo install --git https://github.com/curiosusJR/todo.git --branch main
```


## Usage

```sh-session
$ todo help
simple command-line todo list

Usage: todo <COMMAND>

Commands:
  list        show todo list
  clear       clear todo list
  add         add the task
  delete      delete the task
  edit        edit the task description
  done        done the task and record elapsed time
  undone      undone the task
  begin       record the beginning time point
  unrecord    unrecord elapsed time (same to undone)
  show        show the task
  sort        sort tasks
  swap        swap two tasks
  report      report today's achievements
  continue    continue todo list
  uncontinue  uncontinue todo list
  completion  print shell completion
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```




Add your todo

```sh-session
$ todo add --help
add the task

Usage: todo add <TASK>

Arguments:
  <TASK>

Options:
  -h, --help  Print help
$ todo add 'drink water'
$ todo add 'do some sports'
$ todo add 'laugh 10 times'

```

List todo

```sh-session
$ todo list --help
show todo list

Usage: todo list [PART]

Arguments:
  [PART]  [possible values: doing, done, todo, all]

Options:
  -h, --help  Print help

$ todo list
☐ 000: drink water
☐ 001: do some sports 
☐ 002: laugh 10 times
```

Edit todo

```
$ todo edit 1 'fix bugs'
$ todo list
☐ 000: drink water
☐ 001: fix bugs 
☐ 002: laugh 10 times
```
```

Begin todo with current or given time point

```sh-session
# assume current time is 22:30
$ todo begin 0 
$ todo begin 1 10
$ todo list
☐ 000: drink water (-22.5)
☐ 001: fix bugs (-10.0)
☐ 002: laugh 10 times

```

Done todo and record elapsed time

```sh-session
# assume current time is 23:30
# unbegun todo will done with 0h
$ todo done 0
$ todo done 1
$ todo done 2
$ todo list
☑ 000: drink water (1.0)
☑ 001: fix bugs (7.5)
☑ 002: laugh 10 times (0.0)

```
You can re-begin a done task
If your todos have different situations, 
`todo list` command will automaticly sort them by 'doing-todo-done' order.
```sh-session
$ todo list
☐ 003: buy a keyboard (-7.9)
☐ 001: fix bugs
☑ 000: drink water (0.0)
☑ 002: laugh 10 times (10.0)
```



Report today's achievements

```sh-session

$ todo report --help
report today's achievements

Usage: todo report [OPTIONS] [COMMENT] [TITLE]

Arguments:
  [COMMENT]
  [TITLE]

Options:
  -l, --lang <LANG>  [possible values: ja, en, zh]
  -h, --help         Print help

$ todo report 
## 2024/08/03 (10.0h)
### Doing tasks
- buy a keyboard (-7.9h)

### Done tasks
- drink water (0.0h)
- laugh 10 times (10.0h)

### Todo tasks in this week
- fix bugs

### Memo & Comments
```
```sh-session
$ todo report --lang zh 开心每一天
## 2024/08/03 (10.0h)
### 进行中的任务
- buy a keyboard (-7.9h)

### 已完成的任务
- drink water (0.0h)
- laugh 10 times (10.0h)

### 本周的任务
- fix bugs

### 备忘
开心每一天
```
```sh-session
$ todo report -l ja '' "happy day""
## happy day (10.0h)
### 進行中のタスク
- buy a keyboard (-7.9h)

### 完了済みのタスク
- drink water (0.0h)
- laugh 10 times (10.0h)

### その他、今週対応予定のタスク
- fix bugs

### メモ、ぼやき

```

Continue todo list

```sh-session
$ todo continue
$ todo list
☐ 000: fix bugs
☐ 001: buy a keyboard

```


## Shell Completion

You can load the file to do the bash completion.

```sh-session
$ echo "eval \"\$(todo completion)\"" >> ~/.bash_profile
```

## Interface with Mac shortcuts and message_visualizer
1. Install todo executable file into your PATH
2. Install mac_message_visualizer <https://github.com/curiosusJR/mac-msg-visualizer.github> and build it with Xcode
3. Open the 'todo-add-interface.shortcut' in macOS Shortcut app and edit mac_message_visualizer path in it.
4. Binding a shortcut key with 'shortcuts run todo-add-interface'
5. Test your runnng.

## Note

`todo` is inspired by:

- https://github.com/todotxt/todo.txt-cli
- https://github.com/mattn/todo


## License

The CLI is available as open source under the terms of the [MIT License](http://opensource.org/licenses/MIT).
