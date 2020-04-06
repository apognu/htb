# htb - command-line interface to Hack The Box

`htb` is a command-line client to [Hack The Box](https://www.hackthebox.eu).

![Screenshot of htb](https://repository-images.githubusercontent.com/253286330/92358b00-782f-11ea-96cf-e55b6374a079)

## Machines

### Search machines

Machines can be listed with filters applied to find the find you are looking for. Currently, the following filters are supported:

 * `--owned`: display machines where you owned both _user_ and _root_
 * `--unowned`: display machines you did not complete
 * `--spawned`: display machines that are currently started
 * `--active`: display active machines
 * `--retired`: display retired machines
 * `--todo`: display the machines that you added to your to-do list
 * `--assigned`: display the machines that are assigned to you
 * `--name NAME`: display the machines matching the given name

**Note:** `machines` is aliased to `machine`, `box` and `vm`.

```shell
$ htb machines list --owned --retired
NAME      | OS      | USER     | ROOT     | IP ADDRESS   | RATING | POINTS
Lame      | Linux   | 👥 15430 | ＃ 16559 | 10.10.10.3   | ☆ 4.3  | 🞋 20
Legacy    | Windows | 👥 12370 | ＃ 12860 | 10.10.10.4   | ☆ 4.2  | 🞋 20
Devel     | Windows | 👥 10626 | ＃ 11128 | 10.10.10.5   | ☆ 4.0  | 🞋 20
Beep      | Linux   | 👥 7212  | ＃ 7606  | 10.10.10.7   | ☆ 4.7  | 🞋 20
Optimum   | Windows | 👥 11634 | ＃ 7446  | 10.10.10.8   | ☆ 4.7  | 🞋 20
Grandpa   | Windows | 👥 5104  | ＃ 5257  | 10.10.10.14  | ☆ 3.8  | 🞋 20
Granny    | Windows | 👥 4365  | ＃ 4523  | 10.10.10.15  | ☆ 3.2  | 🞋 20
Blocky    | Linux   | 👥 4322  | ＃ 4312  | 10.10.10.37  | ☆ 4.7  | 🞋 20
Blue      | Windows | 👥 12210 | ＃ 12634 | 10.10.10.40  | ☆ 4.5  | 🞋 20
Mirai     | Linux   | 👥 7235  | ＃ 6858  | 10.10.10.48  | ☆ 4.2  | 🞋 20
[...]
```

### Display machine details

```
$ htb machines show registry
Registry
■■■■■■■■■■■■■■■■■■■■
Hard 💻 Linux - 🞋 40 - 👥 3657 - ＃ 2968

Maker: thek
IP address: 10.10.10.159

First bloods:
  👥 InfoSecJack in 00 days, 03 hours, 28 mins, 17 seconds
  ＃ jkr in 00 days, 04 hours, 26 mins, 08 seconds
```

### Add and remove machines from your to-do list

```shell
$ htb machines todo blue
✓ Blue was added to your todo list
$ htb machines todo blue
✓ Blue was removed from your todo list
```

### Submit flags with difficulty

```shell
$ htb machines own themachine -f e825b7d1941c15cee7512238715f50ff -d 3
✗ Incorrect flag!
```

### Start, stop and reset machines

```shell
$ htb machines start registry
🕑 please wait while we try and assign `Registry` to you...
✓  Machine deployed to lab.

$ htb machines start lame
🕑 please wait while we try and assign `Lame` to you...
✗  You already have an active machine.

$ htb machines reset mirai
🕑 please wait while we try and reset `Mirai`...
✗  Mirai was not reset. Another reset from this user is pending.
```

## Conversations

### List conversations

```shell
$ htb chat list
💬 user1 (123)
   user1: Why?

💬 user2 (456)
   You: Did you start ForwardSlash?
```

### Show messages from conversations

```shell
$ htb chat show 123
👥 user1 (Pro Hacker) - 03:27am
   Lorem ipsum dolor sit amet, consectetur adipiscing elit.

👥 user1 (Pro Hacker) - 03:35am
   Ut vulputate sit amet neque et aliquam. Vestibulum ac interdum dui, eu placerat lectus. Phasellus in risus velit.

👥 user1 (Pro Hacker) - 03:36am
   :sweat_smile:

👥 apognu (Pro Hacker) - 01:39pm
   Vestibulum sollicitudin ullamcorper neque non pharetra. Integer at ipsum ut mauris lobortis semper. Praesent ut erat ac ligula vehicula vulputate vitae at mauris.

👥 apognu (Pro Hacker) - 01:40pm
   Phasellus consequat mi eu augue aliquam placerat.
```

### Open an interactive shell to a chat

```shell
$ htb chat open 123
✓ Chat session started
  /history to display latest messages
  /quit or ^D to exit
05:04pm user1: Lorem ipsum dolor sit amet, consectetur adipiscing elit.
05:10pm apognu: Ut vulputate sit amet neque et aliquam. Vestibulum ac interdum dui, eu placerat lectus.
05:14pm apognu: Phasellus consequat mi eu augue aliquam placerat.
> your message...
```