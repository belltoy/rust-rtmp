use super::PublishRequestType;

#[derive(Debug)]
pub enum TransactionPurpose {
    PlayRequest {
        stream_key: String,
    },

    PublishRequest {
        stream_key: String,
        request_type: PublishRequestType,
    }
}

#[derive(Debug)]
pub enum OutstandingTransaction {
    ConnectionRequested {
        app_name: String,
    },

    CreateStream {
        purpose: TransactionPurpose,
    },
}
