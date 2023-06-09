use quinn::{ConnectionError, ReadError, ReadToEndError, WriteError};
use std::array::TryFromSliceError;
use std::net::AddrParseError;
use std::num::ParseIntError;
use std::str::Utf8Error;
use thiserror::Error;
use tokio::io;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Empty response")]
    EmptyResponse,
    #[error("Invalid configuration")]
    InvalidConfiguration,
    #[error("Not connected")]
    NotConnected,
    #[error("Request error")]
    RequestError(#[from] reqwest::Error),
    #[error("HTTP response error, status: {0}, body: {1}")]
    HttpResponseError(u16, String),
    #[error("Request middleware error")]
    RequestMiddlewareError(#[from] reqwest_middleware::Error),
    #[error("Cannot create endpoint")]
    CannotCreateEndpoint,
    #[error("Cannot parse URL")]
    CannotParseUrl,
    #[error("Invalid response: {0}")]
    InvalidResponse(u8),
    #[error("Cannot parse address")]
    CannotParseAddress(#[from] AddrParseError),
    #[error("Read error")]
    ReadError(#[from] ReadError),
    #[error("Connection error")]
    ConnectionError(#[from] ConnectionError),
    #[error("Read to end error")]
    ReadToEndError(#[from] ReadToEndError),
    #[error("Error")]
    Error,
    #[error("IO error")]
    IoError(#[from] io::Error),
    #[error("Write error")]
    WriteError(#[from] WriteError),
    #[error("Cannot parse integer")]
    CannotParseInt(#[from] ParseIntError),
    #[error("Cannot parse integer")]
    CannotParseSlice(#[from] TryFromSliceError),
    #[error("Cannot parse UTF8")]
    CannotParseUtf8(#[from] Utf8Error),
    #[error("Invalid command")]
    InvalidCommand,
    #[error("Invalid format")]
    InvalidFormat,
    #[error("Cannot create base directory")]
    CannotCreateBaseDirectory,
    #[error("Cannot create streams directory")]
    CannotCreateStreamsDirectory,
    #[error("Cannot create stream with ID: {0} directory")]
    CannotCreateStreamDirectory(u32),
    #[error("Failed to create stream info file for stream with ID: {0}")]
    CannotCreateStreamInfo(u32),
    #[error("Failed to update stream info for stream with ID: {0}")]
    CannotUpdateStreamInfo(u32),
    #[error("Failed to open stream info file for stream with ID: {0}")]
    CannotOpenStreamInfo(u32),
    #[error("Failed to read stream info file for stream with ID: {0}")]
    CannotReadStreamInfo(u32),
    #[error("Cannot read streams")]
    CannotReadStreams,
    #[error("Failed to create stream with ID: {0}")]
    CannotCreateStream(u32),
    #[error("Failed to delete stream with ID: {0}")]
    CannotDeleteStream(u32),
    #[error("Failed to delete stream directory with ID: {0}")]
    CannotDeleteStreamDirectory(u32),
    #[error("Stream with ID: {0} was not found.")]
    StreamNotFound(u32),
    #[error("Stream with ID: {0} already exists.")]
    StreamAlreadyExists(u32),
    #[error("Invalid stream ID")]
    InvalidStreamId,
    #[error("Invalid stream name")]
    InvalidStreamName,
    #[error("Cannot read topics for stream with ID: {0}")]
    CannotReadTopics(u32),
    #[error("Cannot create topics directory for stream with ID: {0}")]
    CannotCreateTopicsDirectory(u32),
    #[error("Failed to create directory for topic with ID: {0} for stream with ID: {1}.")]
    CannotCreateTopicDirectory(u32, u32),
    #[error("Failed to create topic info file for topic with ID: {0} for stream with ID: {1}.")]
    CannotCreateTopicInfo(u32, u32),
    #[error("Failed to update topic info for topic with ID: {0} for stream with ID: {1}.")]
    CannotUpdateTopicInfo(u32, u32),
    #[error("Failed to open topic info file for topic with ID: {0} for stream with ID: {1}.")]
    CannotOpenTopicInfo(u32, u32),
    #[error("Failed to read topic info file for topic with ID: {0} for stream with ID: {1}.")]
    CannotReadTopicInfo(u32, u32),
    #[error("Failed to create topic with ID: {0} for stream with ID: {1}.")]
    CannotCreateTopic(u32, u32),
    #[error("Failed to delete topic with ID: {0} for stream with ID: {1}.")]
    CannotDeleteTopic(u32, u32),
    #[error("Failed to delete topic directory with ID: {0} for stream with ID: {1}.")]
    CannotDeleteTopicDirectory(u32, u32),
    #[error("Cannot poll topic")]
    CannotPollTopic,
    #[error("Topic with ID: {0} for stream with ID: {1} was not found.")]
    TopicNotFound(u32, u32),
    #[error("Topic with ID: {0} for stream with ID: {1} already exists.")]
    TopicAlreadyExists(u32, u32),
    #[error("Invalid topic ID")]
    InvalidTopicId,
    #[error("Invalid topic name")]
    InvalidTopicName,
    #[error("Invalid topic partitions")]
    InvalidTopicPartitions,
    #[error("Log file not found")]
    LogFileNotFound,
    #[error("Cannot append message")]
    CannotAppendMessage,
    #[error("Cannot create partition with ID: {0} for stream with ID: {1} and topic with ID: {2}")]
    CannotCreatePartition(u32, u32, u32),
    #[error("Failed to create directory for partition with ID: {0} for stream with ID: {1} and topic with ID: {2}")]
    CannotCreatePartitionDirectory(u32, u32, u32),
    #[error(
        "Failed to create directory for partitions for stream with ID: {0} and topic with ID: {1}"
    )]
    CannotCreatePartitionsDirectory(u32, u32),
    #[error(
    "Failed to create directory for consumer groups for stream with ID: {0} and topic with ID: {1}"
    )]
    CannotCreateConsumerGroupsDirectory(u32, u32),
    #[error(
        "Failed to delete partition with ID: {0} for stream with ID: {1} and topic with ID: {2}"
    )]
    CannotDeletePartition(u32, u32, u32),
    #[error("Failed to delete partition directory with ID: {0} for stream with ID: {1} and topic with ID: {2}")]
    CannotDeletePartitionDirectory(u32, u32, u32),
    #[error("Failed to create partition segment log file for path: {0}.")]
    CannotCreatePartitionSegmentLogFile(String),
    #[error("Failed to create partition segment index file for path: {0}.")]
    CannotCreatePartitionSegmentIndexFile(String),
    #[error("Failed to create partition segment time index file for path: {0}.")]
    CannotCreatePartitionSegmentTimeIndexFile(String),
    #[error("Cannot open partition log file")]
    CannotOpenPartitionLogFile,
    #[error(
        "Failed to read partitions directories for topic with ID: {0} and stream with ID: {1}"
    )]
    CannotReadPartitions(u32, u32),
    #[error("Partition with ID: {0} was not found.")]
    PartitionNotFound(u32),
    #[error("Invalid messages count")]
    InvalidMessagesCount,
    #[error("Invalid message payload length")]
    InvalidMessagePayloadLength,
    #[error("Segment not found")]
    SegmentNotFound,
    #[error("Segment with start offset: {0} and partition with ID: {1} is closed")]
    SegmentClosed(u64, u32),
    #[error("Segment size is invalid")]
    InvalidSegmentSize(u64),
    #[error("Cannot read message")]
    CannotReadMessage,
    #[error("Cannot read message timestamp")]
    CannotReadMessageTimestamp,
    #[error("Cannot read message ID")]
    CannotReadMessageId,
    #[error("Cannot read message checksum")]
    CannotReadMessageChecksum,
    #[error("Invalid message checksum: {0}, expected: {1}, for offset: {2}")]
    InvalidMessageChecksum(u32, u32, u64),
    #[error("Cannot read message length")]
    CannotReadMessageLength,
    #[error("Cannot read message payload")]
    CannotReadMessagePayload,
    #[error("Cannot save messages to segment")]
    CannotSaveMessagesToSegment,
    #[error("Cannot save index to segment")]
    CannotSaveIndexToSegment,
    #[error("Cannot save time index to segment")]
    CannotSaveTimeIndexToSegment,
    #[error("Empty message payload")]
    EmptyMessagePayload,
    #[error("Too big message payload")]
    TooBigMessagePayload,
    #[error("Too many messages")]
    TooManyMessages,
    #[error("Invalid offset: {0}")]
    InvalidOffset(u64),
    #[error("Failed to read consumers offsets  for partition with ID: {0}")]
    CannotReadConsumerOffsets(u32),
    #[error("Failed to read consumer groups for topic with ID: {0} and stream with ID: {1}")]
    CannotReadConsumerGroups(u32, u32),
    #[error("Failed to create consumer group info file for ID: {0} for topic with ID: {1} for stream with ID: {2}.")]
    CannotCreateConsumerGroupInfo(u32, u32, u32),
    #[error("Failed to delete consumer group info file for ID: {0} for topic with ID: {1} for stream with ID: {2}.")]
    CannotDeleteConsumerGroupInfo(u32, u32, u32),
    #[error("Consumer group with ID: {0} for topic with ID: {1} was not found.")]
    ConsumerGroupNotFound(u32, u32),
    #[error("Consumer group with ID: {0} for topic with ID: {1} already exists.")]
    ConsumerGroupAlreadyExists(u32, u32),
    #[error("Consumer group member with ID: {0} for group with ID: {1} for topic with ID: {2} was not found.")]
    ConsumerGroupMemberNotFound(u32, u32, u32),
    #[error("Invalid consumer group ID")]
    InvalidConsumerGroupId,
    #[error("Feature is unavailable")]
    FeatureUnavailable,
    #[error("Client with ID: {0} was not found.")]
    ClientNotFound(u32),
    #[error("Invalid client ID")]
    InvalidClientId,
}

impl Error {
    pub fn as_code(&self) -> u8 {
        match self {
            Error::Error => 0,
            Error::IoError(_) => 1,
            Error::InvalidCommand => 2,
            Error::InvalidFormat => 3,
            Error::CannotCreateBaseDirectory => 4,
            Error::CannotCreateStreamsDirectory => 5,
            Error::CannotCreateStreamDirectory(_) => 6,
            Error::CannotCreateStreamInfo(_) => 7,
            Error::CannotUpdateStreamInfo(_) => 8,
            Error::CannotOpenStreamInfo(_) => 9,
            Error::CannotReadStreamInfo(_) => 10,
            Error::CannotCreateStream(_) => 11,
            Error::CannotDeleteStream(_) => 12,
            Error::CannotDeleteStreamDirectory(_) => 13,
            Error::StreamNotFound(_) => 14,
            Error::StreamAlreadyExists(_) => 15,
            Error::InvalidStreamName => 16,
            Error::CannotCreateTopicsDirectory(_) => 17,
            Error::CannotCreateTopicDirectory(_, _) => 18,
            Error::CannotCreateTopicInfo(_, _) => 19,
            Error::CannotUpdateTopicInfo(_, _) => 20,
            Error::CannotOpenTopicInfo(_, _) => 21,
            Error::CannotReadTopicInfo(_, _) => 22,
            Error::CannotCreateTopic(_, _) => 23,
            Error::CannotDeleteTopic(_, _) => 24,
            Error::CannotDeleteTopicDirectory(_, _) => 25,
            Error::CannotPollTopic => 26,
            Error::TopicNotFound(_, _) => 27,
            Error::TopicAlreadyExists(_, _) => 28,
            Error::InvalidTopicName => 29,
            Error::InvalidTopicPartitions => 30,
            Error::LogFileNotFound => 31,
            Error::CannotAppendMessage => 32,
            Error::CannotCreatePartition(_, _, _) => 33,
            Error::CannotCreatePartitionDirectory(_, _, _) => 34,
            Error::CannotCreatePartitionSegmentLogFile(_) => 35,
            Error::CannotCreatePartitionSegmentIndexFile(_) => 36,
            Error::CannotCreatePartitionSegmentTimeIndexFile(_) => 37,
            Error::CannotOpenPartitionLogFile => 38,
            Error::CannotReadPartitions(_, _) => 39,
            Error::PartitionNotFound(_) => 40,
            Error::InvalidMessagesCount => 41,
            Error::InvalidStreamId => 42,
            Error::InvalidTopicId => 43,
            Error::SegmentNotFound => 44,
            Error::SegmentClosed(_, _) => 45,
            Error::InvalidSegmentSize(_) => 46,
            Error::CannotReadMessage => 47,
            Error::CannotReadMessageTimestamp => 48,
            Error::CannotReadMessageId => 49,
            Error::CannotReadMessageLength => 50,
            Error::CannotReadMessagePayload => 51,
            Error::CannotSaveMessagesToSegment => 52,
            Error::CannotSaveIndexToSegment => 53,
            Error::CannotSaveTimeIndexToSegment => 54,
            Error::CannotParseUtf8(_) => 55,
            Error::CannotParseInt(_) => 56,
            Error::CannotParseSlice(_) => 57,
            Error::TooBigMessagePayload => 58,
            Error::TooManyMessages => 59,
            Error::WriteError(_) => 60,
            Error::InvalidOffset(_) => 61,
            Error::CannotReadConsumerOffsets(_) => 62,
            Error::CannotDeletePartition(_, _, _) => 63,
            Error::CannotDeletePartitionDirectory(_, _, _) => 64,
            Error::InvalidMessagePayloadLength => 65,
            Error::EmptyMessagePayload => 67,
            Error::CannotReadStreams => 68,
            Error::CannotReadTopics(_) => 69,
            Error::CannotReadMessageChecksum => 70,
            Error::InvalidMessageChecksum(_, _, _) => 71,
            Error::ConsumerGroupNotFound(_, _) => 72,
            Error::ConsumerGroupAlreadyExists(_, _) => 73,
            Error::ConsumerGroupMemberNotFound(_, _, _) => 74,
            Error::InvalidConsumerGroupId => 75,
            Error::FeatureUnavailable => 76,
            Error::CannotCreatePartitionsDirectory(_, _) => 77,
            Error::CannotCreateConsumerGroupsDirectory(_, _) => 78,
            Error::CannotReadConsumerGroups(_, _) => 79,
            Error::CannotCreateConsumerGroupInfo(_, _, _) => 80,
            Error::CannotDeleteConsumerGroupInfo(_, _, _) => 81,
            Error::ClientNotFound(_) => 82,
            Error::InvalidClientId => 83,
            _ => 255,
        }
    }

    pub fn as_text_code(&self) -> &'static str {
        match self {
            Error::Error => "error",
            Error::IoError(_) => "io_error",
            Error::InvalidCommand => "invalid_command",
            Error::InvalidFormat => "invalid_format",
            Error::CannotCreateBaseDirectory => "cannot_create_base_directory",
            Error::CannotCreateStreamsDirectory => "cannot_create_streams_directory",
            Error::CannotCreateStreamDirectory(_) => "cannot_create_stream_directory",
            Error::CannotCreateStreamInfo(_) => "cannot_create_stream_info",
            Error::CannotUpdateStreamInfo(_) => "cannot_update_stream_info",
            Error::CannotOpenStreamInfo(_) => "cannot_open_stream_info",
            Error::CannotReadStreamInfo(_) => "cannot_read_stream_info",
            Error::CannotCreateStream(_) => "cannot_create_stream",
            Error::CannotDeleteStream(_) => "cannot_delete_stream",
            Error::CannotDeleteStreamDirectory(_) => "cannot_delete_stream_directory",
            Error::StreamNotFound(_) => "stream_not_found",
            Error::StreamAlreadyExists(_) => "stream_already_exists",
            Error::InvalidStreamName => "invalid_stream_name",
            Error::CannotCreateTopicsDirectory(_) => "cannot_create_topics_directory",
            Error::CannotCreateTopicDirectory(_, _) => "cannot_create_topic_directory",
            Error::CannotCreateTopicInfo(_, _) => "cannot_create_topic_info",
            Error::CannotUpdateTopicInfo(_, _) => "cannot_update_topic_info",
            Error::CannotOpenTopicInfo(_, _) => "cannot_open_topic_info",
            Error::CannotReadTopicInfo(_, _) => "cannot_read_topic_info",
            Error::CannotCreateTopic(_, _) => "cannot_create_topic",
            Error::CannotDeleteTopic(_, _) => "cannot_delete_topic",
            Error::CannotDeleteTopicDirectory(_, _) => "cannot_delete_topic_directory",
            Error::CannotPollTopic => "cannot_poll_topic",
            Error::TopicNotFound(_, _) => "topic_not_found",
            Error::TopicAlreadyExists(_, _) => "topic_already_exists",
            Error::InvalidTopicName => "invalid_topic_name",
            Error::InvalidTopicPartitions => "invalid_topic_partitions",
            Error::LogFileNotFound => "log_file_not_found",
            Error::CannotAppendMessage => "cannot_append_message",
            Error::CannotCreatePartition(_, _, _) => "cannot_create_partition",
            Error::CannotCreatePartitionDirectory(_, _, _) => "cannot_create_partition_directory",
            Error::CannotCreatePartitionSegmentLogFile(_) => {
                "cannot_create_partition_segment_log_file"
            }
            Error::CannotCreatePartitionSegmentIndexFile(_) => {
                "cannot_create_partition_segment_index_file"
            }
            Error::CannotCreatePartitionSegmentTimeIndexFile(_) => {
                "cannot_create_partition_segment_time_index_file"
            }
            Error::CannotOpenPartitionLogFile => "cannot_open_partition_log_file",
            Error::CannotReadPartitions(_, _) => "cannot_read_partitions",
            Error::PartitionNotFound(_) => "partition_not_found",
            Error::InvalidMessagesCount => "invalid_messages_count",
            Error::InvalidStreamId => "invalid_stream_id",
            Error::InvalidTopicId => "invalid_topic_id",
            Error::SegmentNotFound => "segment_not_found",
            Error::SegmentClosed(_, _) => "segment_closed",
            Error::InvalidSegmentSize(_) => "invalid_segment_size",
            Error::CannotReadMessage => "cannot_read_message",
            Error::CannotReadMessageTimestamp => "cannot_read_message_timestamp",
            Error::CannotReadMessageId => "cannot_read_message_id",
            Error::CannotReadMessageLength => "cannot_read_message_length",
            Error::CannotReadMessagePayload => "cannot_read_message_payload",
            Error::CannotSaveMessagesToSegment => "cannot_save_messages_to_segment",
            Error::CannotSaveIndexToSegment => "cannot_save_index_to_segment",
            Error::CannotSaveTimeIndexToSegment => "cannot_save_time_index_to_segment",
            Error::CannotParseUtf8(_) => "cannot_parse_utf8",
            Error::CannotParseInt(_) => "cannot_parse_int",
            Error::CannotParseSlice(_) => "cannot_parse_slice",
            Error::TooBigMessagePayload => "too_big_message_payload",
            Error::TooManyMessages => "too_many_messages",
            Error::WriteError(_) => "write_error",
            Error::InvalidOffset(_) => "invalid_offset",
            Error::CannotReadConsumerOffsets(_) => "cannot_read_consumer_offsets",
            Error::CannotDeletePartition(_, _, _) => "cannot_delete_partition",
            Error::CannotDeletePartitionDirectory(_, _, _) => "cannot_delete_partition_directory",
            Error::InvalidMessagePayloadLength => "invalid_message_payload_length",
            Error::EmptyMessagePayload => "empty_message_payload",
            Error::CannotReadStreams => "cannot_read_streams",
            Error::CannotReadTopics(_) => "cannot_read_topics",
            Error::CannotReadMessageChecksum => "cannot_read_message_checksum",
            Error::InvalidMessageChecksum(_, _, _) => "invalid_message_checksum",
            Error::ConsumerGroupNotFound(_, _) => "consumer_group_not_found",
            Error::ConsumerGroupAlreadyExists(_, _) => "consumer_group_already_exists",
            Error::ConsumerGroupMemberNotFound(_, _, _) => "consumer_group_member_not_found",
            Error::FeatureUnavailable => "feature_unavailable",
            Error::CannotCreatePartitionsDirectory(_, _) => "cannot_create_partitions_directory",
            Error::CannotCreateConsumerGroupsDirectory(_, _) => {
                "cannot_create_consumer_groups_directory"
            }
            Error::CannotReadConsumerGroups(_, _) => "cannot_read_consumer_groups",
            Error::CannotCreateConsumerGroupInfo(_, _, _) => "cannot_create_consumer_group_info",
            Error::CannotDeleteConsumerGroupInfo(_, _, _) => "cannot_delete_consumer_group_info",
            Error::ClientNotFound(_) => "client_not_found",
            Error::InvalidClientId => "invalid_client_id",
            _ => "error",
        }
    }
}
