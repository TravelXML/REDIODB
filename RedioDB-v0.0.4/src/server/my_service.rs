// src/server/my_service.rs
//
// The gRPC service implementation for EdgeDB.
// Implements the generated Rediodb trait using TTLStore and stubs for extended features.

use std::pin::Pin;
use std::sync::Mutex;
use std::time::Duration;
use tonic::{Request, Response, Status};
use futures_core::Stream;
use futures_util::stream::empty;
use lazy_static::lazy_static;

use crate::consensus::raft::RaftNode;
use crate::query::engine::QueryEngine;
use crate::ai::inference::InferenceEngine;
use crate::storage::ttl_store::TTLStore;
use crate::server::rediodb_server::rediodb_server::Rediodb;
use crate::server::rediodb_server::{
    // Basic operations
    QueryRequest, QueryResponse, SetRequest, ResponseMessage, KeyRequest, ValueResponse, ExpireRequest, TtlResponse,
    // Atomic operations
    IncrRequest, DecrRequest, AppendRequest,
    // Pattern matching
    PatternRequest, KeysResponse,
    // Transactions
    MultiRequest, ExecRequest,
    // List operations
    ListPushRequest, ListPopRequest,
    // Set operations
    SetAddRequest, SetMembersRequest, SetMembersResponse,
    // Hash operations
    HashSetRequest, HashGetRequest,
    // Pub/Sub
    PublishRequest, SubscribeRequest, PubSubMessage,
};

lazy_static! {
    static ref RAFT_NODE: Mutex<RaftNode> = Mutex::new(RaftNode::new());
    static ref QUERY_ENGINE: Mutex<QueryEngine> = Mutex::new(QueryEngine::new());
    static ref INFERENCE_ENGINE: Mutex<InferenceEngine> = Mutex::new(InferenceEngine::new("model.onnx"));
    static ref STORAGE: Mutex<TTLStore> = Mutex::new(TTLStore::new());
}

/// MyService implements the Rediodb gRPC trait.
#[derive(Default)]
pub struct MyService {}

#[tonic::async_trait]
impl Rediodb for MyService {
    // Basic Query Execution (if applicable)
    async fn execute(
        &self,
        request: Request<QueryRequest>,
    ) -> Result<Response<QueryResponse>, Status> {
        let req = request.into_inner();
        let query_text = req.query.unwrap_or_default().query;
        if !RAFT_NODE.lock().unwrap().propose(&query_text) {
            return Err(Status::internal("Raft proposal failed"));
        }
        let result = QUERY_ENGINE.lock().unwrap().execute(&query_text);
        let _ = INFERENCE_ENGINE.lock().unwrap().infer(&query_text);
        STORAGE.lock().unwrap().set("last_query", &query_text, None);
        Ok(Response::new(QueryResponse { result }))
    }

    // Basic Key-Value Operations
    async fn set(
        &self,
        request: Request<SetRequest>,
    ) -> Result<Response<ResponseMessage>, Status> {
        let req = request.into_inner();
        let ttl_duration = if req.ttl > 0 {
            Some(Duration::from_secs(req.ttl as u64))
        } else {
            None
        };
        STORAGE.lock().unwrap().set(&req.key, &req.value, ttl_duration);
        let reply = ResponseMessage {
            status: "success".into(),
            message: format!("Key '{}' set successfully", req.key),
        };
        Ok(Response::new(reply))
    }

    async fn get(
        &self,
        request: Request<KeyRequest>,
    ) -> Result<Response<ValueResponse>, Status> {
        let key = request.into_inner().key;
        let value = STORAGE.lock().unwrap().get(&key).unwrap_or_default();
        Ok(Response::new(ValueResponse { value }))
    }

    async fn expire(
        &self,
        request: Request<ExpireRequest>,
    ) -> Result<Response<ResponseMessage>, Status> {
        let req = request.into_inner();
        let ttl_duration = Duration::from_secs(req.ttl as u64);
        if STORAGE.lock().unwrap().expire(&req.key, ttl_duration) {
            let reply = ResponseMessage {
                status: "success".into(),
                message: format!("TTL for key '{}' set to {} seconds", req.key, req.ttl),
            };
            Ok(Response::new(reply))
        } else {
            Err(Status::not_found("Key not found"))
        }
    }

    async fn ttl(
        &self,
        request: Request<KeyRequest>,
    ) -> Result<Response<TtlResponse>, Status> {
        let key = request.into_inner().key;
        let ttl_value = STORAGE.lock().unwrap().ttl(&key).unwrap_or(-1);
        Ok(Response::new(TtlResponse { ttl: ttl_value }))
    }

    async fn del(
        &self,
        request: Request<KeyRequest>,
    ) -> Result<Response<ResponseMessage>, Status> {
        let key = request.into_inner().key;
        if STORAGE.lock().unwrap().del(&key) {
            let reply = ResponseMessage {
                status: "success".into(),
                message: format!("Key '{}' deleted", key),
            };
            Ok(Response::new(reply))
        } else {
            Err(Status::not_found("Key not found"))
        }
    }

    // Extended Atomic Operations
    async fn incr(
        &self,
        request: Request<IncrRequest>,
    ) -> Result<Response<ValueResponse>, Status> {
        let req = request.into_inner();
        let new_val = STORAGE
            .lock()
            .unwrap()
            .incr(&req.key, req.amount)
            .ok_or_else(|| Status::internal("Failed to increment key"))?;
        Ok(Response::new(ValueResponse { value: new_val }))
    }

    async fn decr(
        &self,
        request: Request<DecrRequest>,
    ) -> Result<Response<ValueResponse>, Status> {
        let req = request.into_inner();
        let new_val = STORAGE
            .lock()
            .unwrap()
            .decr(&req.key, req.amount)
            .ok_or_else(|| Status::internal("Failed to decrement key"))?;
        Ok(Response::new(ValueResponse { value: new_val }))
    }

    async fn append(
        &self,
        request: Request<AppendRequest>,
    ) -> Result<Response<ValueResponse>, Status> {
        let req = request.into_inner();
        let new_val = STORAGE
            .lock()
            .unwrap()
            .append(&req.key, &req.value)
            .ok_or_else(|| Status::internal("Failed to append to key"))?;
        Ok(Response::new(ValueResponse { value: new_val }))
    }

    // Key Pattern Matching
    async fn keys(
        &self,
        request: Request<PatternRequest>,
    ) -> Result<Response<KeysResponse>, Status> {
        let pattern = request.into_inner().pattern;
        let keys = STORAGE.lock().unwrap().keys(&pattern);
        Ok(Response::new(KeysResponse { keys }))
    }

    // Transaction Support
    async fn multi(
        &self,
        request: Request<MultiRequest>,
    ) -> Result<Response<ResponseMessage>, Status> {
        let req = request.into_inner();
        println!("Queuing transaction commands: {:?}", req.commands);
        Ok(Response::new(ResponseMessage {
            status: "success".into(),
            message: "Transaction block queued".into(),
        }))
    }

    async fn exec(
        &self,
        _request: Request<ExecRequest>,
    ) -> Result<Response<ResponseMessage>, Status> {
        Ok(Response::new(ResponseMessage {
            status: "success".into(),
            message: "Transaction executed".into(),
        }))
    }

    // Data Structures: Lists
    async fn l_push(
        &self,
        request: Request<ListPushRequest>,
    ) -> Result<Response<ResponseMessage>, Status> {
        let req = request.into_inner();
        STORAGE.lock().unwrap().l_push(&req.key, &req.value);
        Ok(Response::new(ResponseMessage {
            status: "success".into(),
            message: format!("Value '{}' pushed to list '{}'", req.value, req.key),
        }))
    }

    async fn l_pop(
        &self,
        request: Request<ListPopRequest>,
    ) -> Result<Response<ValueResponse>, Status> {
        let key = request.into_inner().key;
        let popped = STORAGE.lock().unwrap().l_pop(&key).unwrap_or_default();
        Ok(Response::new(ValueResponse { value: popped }))
    }

    // Data Structures: Sets
    async fn s_add(
        &self,
        request: Request<SetAddRequest>,
    ) -> Result<Response<ResponseMessage>, Status> {
        let req = request.into_inner();
        STORAGE.lock().unwrap().s_add(&req.key, &req.member);
        Ok(Response::new(ResponseMessage {
            status: "success".into(),
            message: format!("Member '{}' added to set '{}'", req.member, req.key),
        }))
    }

    async fn s_members(
        &self,
        request: Request<SetMembersRequest>,
    ) -> Result<Response<SetMembersResponse>, Status> {
        let key = request.into_inner().key;
        let members = STORAGE.lock().unwrap().s_members(&key);
        Ok(Response::new(SetMembersResponse { members }))
    }

    // Data Structures: Hashes
    async fn h_set(
        &self,
        request: Request<HashSetRequest>,
    ) -> Result<Response<ResponseMessage>, Status> {
        let req = request.into_inner();
        STORAGE.lock().unwrap().h_set(&req.key, &req.field, &req.value);
        Ok(Response::new(ResponseMessage {
            status: "success".into(),
            message: format!("Field '{}' set for hash '{}'", req.field, req.key),
        }))
    }

    async fn h_get(
        &self,
        request: Request<HashGetRequest>,
    ) -> Result<Response<ValueResponse>, Status> {
        let req = request.into_inner();
        let value = STORAGE.lock().unwrap().h_get(&req.key, &req.field).unwrap_or_default();
        Ok(Response::new(ValueResponse { value }))
    }

    // Enhanced Pub/Sub
    async fn publish(
        &self,
        request: Request<PublishRequest>,
    ) -> Result<Response<ResponseMessage>, Status> {
        let req = request.into_inner();
        println!("Publishing to channel '{}': {}", req.channel, req.message);
        Ok(Response::new(ResponseMessage {
            status: "success".into(),
            message: format!("Message published to channel '{}'", req.channel),
        }))
    }

    async fn subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let _req = request.into_inner();
        let stream = empty();
        Ok(Response::new(Box::pin(stream)))
    }

    type SubscribeStream = SubscribeStream;
}

// Define the SubscribeStream type alias only once as a pinned box.
pub type SubscribeStream = Pin<Box<dyn Stream<Item = Result<PubSubMessage, Status>> + Send + 'static>>;
