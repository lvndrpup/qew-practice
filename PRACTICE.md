# Try qew in 7 steps

Run these in PowerShell from this folder. `.\qew` runs qew offline (Python 3.15); nothing goes to GitHub.

1. `.\qew list` shows one draft waiting for you: "Add greet function".
2. `.\qew draft approve ID --quote "Me: 'yes, build greet first'"` is your yes. Only your words release a draft.
3. `.\qew ready` shows the work you can start now.
4. `.\qew start ID` claims it.
5. Write `greet.py`, then `git add .` and `git commit -m "feat: greet"`.
6. `.\qew evidence ID` ties that commit to the item.
7. `.\qew complete ID` finishes it. Try it before step 6: qew refuses without evidence.

Then look around: `.\qew board`, `.\qew show ID`.
Try with Claude: open Claude Code here and say "file two small tasks in qew, then do the first one".

Ledger (items and history) lives in `..\qew-practice-ledger`. To start over: delete both folders.
