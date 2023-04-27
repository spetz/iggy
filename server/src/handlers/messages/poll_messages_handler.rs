use crate::sender::Sender;
use anyhow::Result;
use shared::error::Error;
use shared::messages::poll_messages::PollMessages;
use streaming::system::System;
use tracing::trace;

/*
    |  POLL   |   STREAM  |   TOPIC   |    PT_ID  |    KIND   |   VALUE   |   COUNT   |
    | 1 byte  |  4 bytes  |  4 bytes  |   4 bytes |   1 byte  |  8 bytes  |  4 bytes  |

    POLL
        - Constant 1 byte of value 2

    STREAM:
        - Unique Stream ID to poll the messages from.

    TOPIC:
        - Unique Topic ID to poll the messages from.

    KIND:
        - 0 -> first
        - 1 -> last
        - 2 -> next
        - 3 -> offset
        - 4 -> timestamp

    VALUE:
        - when KIND is 0, 1, 2, value is ignored
        - when KIND is 3, value is the exact offset
        - when KIND is 4, value is the timestamp (greater than or equal to)

    COUNT:
        - Number of messages to poll in a single chunk.

    Poll the message(s) from stream: 1, topic: 1, partition: 1, using kind: first, value is ignored, messages count is 1.
    |    2    |     1     |     1     |     1     |     0     |     0     |     1     |

    Poll the message(s) stream: 1, topic: 1, partition: 1, using kind: last, value is ignored, messages count is 1.
    |    2    |     1     |     1     |     1     |     1     |     0     |     1     |

    Poll the message(s) stream: 1, topic: 1, partition: 1, using kind: next, value is ignored, messages count is 1.
    |    2    |     1     |     1     |     1     |     2     |     0     |     1     |

    Poll the message(s) stream: 1, topic: 1, partition: 1, using kind: offset, value is 1, messages count is 1.
    |    2    |     1     |     1     |     1     |     3     |     1     |     1     |

    Poll the message(s) stream: 1, topic: 1, partition: 1, using kind: timestamp, value is 1679997285, messages count is 1.
    |    2    |     1     |     1     |     1     |     4     |1679997285 |     1     |
*/

pub async fn handle(
    command: PollMessages,
    sender: &mut Sender,
    system: &mut System,
) -> Result<(), Error> {
    if command.count == 0 {
        return Err(Error::InvalidMessagesCount);
    }

    trace!(
        "Polling {} messages from stream: {:?}, topic: {:?}, kind: {:?}, value: {:?}...",
        command.count,
        command.stream_id,
        command.topic_id,
        command.kind,
        command.value,
    );

    let messages = system.get_stream(command.stream_id)?.get_messages(
        command.topic_id,
        command.partition_id,
        command.value,
        command.count,
    )?;

    let messages_count = messages.len() as u32;
    let messages_size = messages
        .iter()
        .map(|message| message.get_size_bytes())
        .sum::<u32>();

    let mut bytes = Vec::with_capacity(4 + messages_size as usize);
    bytes.extend(messages_count.to_le_bytes());
    for message in messages {
        message.extend(&mut bytes);
    }

    sender.send_ok_response(&bytes).await?;

    trace!(
        "Polled {} message(s) from stream: {}, topic: {:?}, kind: {:?}, value: {:?}, count: {:?}",
        messages_count,
        command.stream_id,
        command.topic_id,
        command.kind,
        command.value,
        command.count
    );
    Ok(())
}
