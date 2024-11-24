# AsyncPlayground
I'm using this repo to test an idea for managing the sync/async boundary,
wherein the runtime is launched from a separate thread and tasks are spawned
by sending Futures through a channel.