# ✨🥸 Sparkle Impostor

A third party crate for [Twilight](https://github.com/twilight-rs/twilight) to execute Discord webhooks that clone an
existing message

## 🤷 Why?

My bots use it, it's open source because why not

## 📦 Cargo Features

- `upload`: Enables methods for re-uploading attachments

## 🧪 Testing

The crate uses integration tests as opposed to unit tests to test real-world usage. It creates a message and clones it,
then the tester checks if the message is cloned as expected

Before starting, set these environment variables, you can also put them in a `.env` file:

- `BOT_TOKEN`: The token of the bot to use for testing
- `CHANNEL_ID`: The channel in which the messages and webhooks will be crated
- `FORUM_CHANNEL_ID`: The forum channel in which cloning messages/threads in forum channels will be tested
- `NOT_LAST_SOURCE_THREAD_ID`: The bot will create a thread and spam to 200 in it the first time the tests are ran, to
  avoid doing this again, set this to the ID of this thread

Required permissions in `CHANNEL_ID` and `FORUM_CHANNEL_ID`:

- `VIEW_CHANNEL`
- `MANAGE_WEBHOOKS`
- `SEND_MESSAGES`

Required additional permissions in `FORUM_CHANNEL_ID`:

- `CREATE_POSTS`
- `SEND_MESSAGES_IN_POSTS`

Test with a single thread to avoid race conditions: `cargo test -- --test-threads=1`