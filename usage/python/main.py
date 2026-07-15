"""Use case: keep a companion on screen while a slow task runs, then react.

You just call the API — no subprocess wrangling on your side.
"""

import time

import awan

job = awan.busy("crunching numbers")  # he works while you do…
time.sleep(2.5)                        # …something slow here
job.terminate()

awan.react("task.done")                # 🎉  (swap for "cmd.failed" on failure)
