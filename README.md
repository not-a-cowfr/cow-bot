<h1 align="center">
cow bot
</h1>

Discord bot made for different miscellaneous skyblock utilities

<h2 align="center">
Features
</h2>

- Get a user's mojang info and some quick links - User context menu only
- Get a user's estimated hypixel uptime - `/uptime`
- Set your own personal custom color for the bot - `/color`
- Feature rich tag system nearly identitical to that of Fire's without any premium subscription
- Link your minecraft account for easier (and faster!) responses - `/link`

<h2 align="center">
Contribute
</h2>

Check out the [Contributing guide](/CONTRIBUTING.md) for more info on how get things set up

<h2 align="center">
    TODO
</h2>

<h4 align="center">
    Features
</h4>

- [ ] Add stats command to display the amount of tracked players and guilds (tag stats? 👀)
- [ ] Add a command to view the highest uptime of tracked players
- [ ] Use elite api graph endpoint to add collection/skill tracking
    - [ ] Add a command similar to `/uptime` for this
    - [ ] Add a command to view the highest collection/skill gain of tracked players
- [ ] Add total hours and average uptime to `/uptime`
- [ ] Add graph to `/uptime`
- [ ] In `/uptime` first send the embed with last saved values form db, and then update it right after with the latest values from api so taht its both a very quick response and also accurate

<h4 align="center">
    Bug fixes
</h4>

- [ ] Fix uptime having duplicated dates

<h4 align="center">
    Improvements
</h4>

- [ ] Optimize uptime command
- [ ] Extract common error structs and abstract them into main.rs, thne use that to improve errro handling
- [ ] Various TODO comments spread all over

<h2 align="center">
Credits
</h2>

- **[Serenity](https://github.com/serenity-rs/serenity/)** - The main library used to interface with discord
- **[Poise](https://github.com/serenity-rs/poise)** - The framework that the bot is built on
- **[Hypixel API](https://api.hypixel.net/)** - Used for nearly all player data
- **[Elite API](https://api.elitebot.dev/)** - Used for player farming data
