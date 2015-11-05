# tasks
How to create tasks, manage tasks and stay productive.

## Creating tasks
A day has 24 hours, of which 8 are for sleep, 8 are for work, and 8 for the
rest. There's only so much time you can be productive in a day, so it's key to
utilize your time efficiently and spend as little time (and mental power) as
possible on side tasks.

A neat strategy to create value for your day is to create an achievable goal
for the day, and write down a max of 5 items that help you achieve that goal.

## Managing tasks
Managing tasks is something you want to spend the minimum amount of time on as
it doesn't produce any direct value. For that purpose I use a program called
`task(1)` (taskwarrior).

#### Create
Tasks can be created and assigned to projects
```txt
$ task add project:myProject +myLabel 'This is my task'
```

#### Read
Tasks can be printed to stdout, filtered by project and further filtered by
label name. You can also specify when a task is due in natural language.
```txt
$ task project:myProject +myLabel due.before: today ls
ID D Project   Due   Description
 1   myProject 1 day create `pull build` bar

1 task
```

#### Update
Tasks can depend on each other
```txt
$ task 1 modify depends:2-4
```

#### Read subcommand
All tasks accept a `<filter>` property.
```sh
task blocked                         # blocking tasks
task active                          # started but not completed
task burndown.{daily,weekly,monthly} # burndown chart
task calendar                        # calendar with due tasks marked
task count                           # amount of tasks matching filter
task export                          # export to JSON
task ghistory.{monthly,annual}       # graphic report of task status
task next                            # most urgent task
task overdue                         # tasks which are beyond the due date
task recurring                       # all recurring tasks
```

#### Write subcommands
All tasks accept a `<filter>` property.
```txt
task add          # add a new task
task annotate     # add an annotation to a task
task append       # append description text to a task
task delete       # delete the task
task denotate     # remove an annotation from a task
task done         # mark a task as done
task duplicate    # duplicate the task and allow modifications
task edit         # edit the task in an editor
task import       # import a task from JSON
task log          # add an already completed task to the list
task modify       # edit an existing task
task prepend      # prepend description text to a task
task start        # mark the specified tasks as started
task stop         # remove the start time from the task
```

#### Misc subcommands
```txt
task context <name>                 # set the active context
task context delete <name>          # delete the context
task context define <name> <filter> # define a new context
task context list                   # print all contexts
task context none                   # unset current context
task context show                   # show the currently active context
task diagnostics                    # show diagnostic information for bug reports
task execute                        # execute an external command
task logo                           # print taskwarrior logo
task reports                        # list all supported reports
task show                           # show current settings
task stats                          # show stats
task timesheet                      # show weekly report of task completed & started
task undo                           # undo the last action
task version                        # show the version
```

#### Attributes & metadata
[tbd]

#### Attribute modifiers
[tbd]

#### Context
[tbd]

#### Regexes
Regexes are kind of tricky. Here's an example to add a new context that matches
a regex:
```sh
$ task context define myProject "project ~ 'myProject/*'"
```

## See also
- [Productivity advice](http://tnw.to/h4tIH)
- [getting things done with taskwarrior](http://chariotsolutions.com/blog/post/getting-things-done-with-task-warrior/)
- [inthe.am](https://inthe.am/about) - task sync server
- [freecinc.com](https://freecinc.com/) - task sync server
