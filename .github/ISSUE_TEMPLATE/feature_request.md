---
name: Feature request
about: Suggest an idea for this project
title: "[Feature]"
labels: feature request
assignees: gabehellz

---

**What is it?**
Describe what is this feature.

Ex.: Moderation command to ban users from the server.

**What does it do? What does it solve?**
Describe what problem this feature solves. Ex.: It bans multiple specified users from the server. The users should be in the server in order to be banned.

**How does it works?**
Describe in details how this feature should work.

Ex.: A moderator executes the `/ban <users...>` command providing the user objects (mention, ID or username) of members that should be banned, separated by space, and they're banned from the server.

**Additional context**
Add any other context or screenshots about the feature request here.

**Verification points**
Add a list of things that should be checked when implementing this feature.

Ex.:
- [ ] The caller (who executes the command) needs `BAN_MEMBERS` permission
- [ ] The users that will be banned must be in the server
- [ ] The command must not execute if any of the user objects is invalid
- [ ] ...
