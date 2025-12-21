# DPI Probe Performance Model

**Path and File Name:** `/home/ransomeye/rebuild/ransomeye_dpi_probe/docs/performance_model.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7OxYQylg8CMw1iGsq7gU  
**Details:** Performance characteristics and scalability model for DPI Probe

---

## Performance Targets

### Throughput
- **Minimum**: 1 Gbps sustained
- **Target**: 10 Gbps sustained
- **Peak**: 40 Gbps (with packet drops)

### Latency
- **Per-packet**: < 1ms processing time
- **Flow export**: < 100ms from flow completion
- **Telemetry transmission**: < 50ms (excluding network)

### Resource Usage
- **Memory**: Bounded by `MAX_BUFFER_SIZE_MB`
- **CPU**: Efficient packet parsing (< 20% on 4-core system)
- **Disk**: Minimal (buffering only)
- **Network**: mTLS connection to Core

## Bottlenecks

### 1. Packet Capture
- **Constraint**: libpcap/AF_PACKET throughput
- **Mitigation**: Zero-copy capture where possible
- **Fallback**: Drop packets if queue full

### 2. Flow Assembly
- **Constraint**: Hash map lookup performance
- **Mitigation**: Efficient hash functions, lock-free structures
- **Scaling**: O(1) flow lookup

### 3. Feature Extraction
- **Constraint**: CPU-intensive metadata extraction
- **Mitigation**: Minimal processing, no AI/ML
- **Optimization**: Pre-computed statistics

### 4. Event Signing
- **Constraint**: RSA-4096 signing performance (~10ms per event)
- **Mitigation**: Batch signing, async processing
- **Limitation**: ~100 events/second signing rate

### 5. Transport
- **Constraint**: Network bandwidth to Core
- **Mitigation**: Compression, batching
- **Backpressure**: Core signals when overloaded

## Scalability Model

### Horizontal Scaling
- **Multiple instances**: One per network interface
- **Independent operation**: No coordination required
- **Load distribution**: Core handles aggregation

### Vertical Scaling
- **CPU**: Linear scaling with cores (multi-threaded capture)
- **Memory**: Bounded by buffer configuration
- **Network**: Limited by interface bandwidth

## Performance Monitoring

### Key Metrics
- **Packets captured/second**: Throughput indicator
- **Flows exported/second**: Flow assembly rate
- **Events signed/second**: Signing throughput
- **Events transmitted/second**: Transport rate
- **Buffer utilization**: Memory usage
- **Drop rate**: Packet/event drops

### Performance Degradation

#### High Packet Rate
- **Symptom**: Packet queue full, packets dropped
- **Mitigation**: Increase queue size, optimize parsing
- **Threshold**: > 80% queue utilization

#### High Flow Count
- **Symptom**: Memory pressure, slow lookups
- **Mitigation**: Reduce flow timeout, cleanup more frequently
- **Threshold**: > 1M active flows

#### Signing Bottleneck
- **Symptom**: Events queued, buffer fills
- **Mitigation**: Batch signing, optimize key operations
- **Threshold**: > 50 events queued for signing

#### Transport Bottleneck
- **Symptom**: Backpressure signals, buffer fills
- **Mitigation**: Increase buffer size, optimize compression
- **Threshold**: > 80% buffer utilization

## Optimization Strategies

### 1. Zero-Copy Capture
- Use AF_PACKET ring buffers
- Minimize packet copying
- Direct memory access

### 2. Lock-Free Data Structures
- DashMap for flow storage
- Atomic counters
- RCU-style updates

### 3. Batch Processing
- Batch event signing
- Batch transport transmission
- Reduce syscall overhead

### 4. Memory Pooling
- Pre-allocate packet buffers
- Reuse flow structures
- Reduce allocations

### 5. Async I/O
- Non-blocking network I/O
- Async file I/O for buffering
- Parallel processing pipelines

## Benchmarking

### Test Scenarios
1. **Low rate**: 100 Mbps, 1K flows/sec
2. **Medium rate**: 1 Gbps, 10K flows/sec
3. **High rate**: 10 Gbps, 100K flows/sec
4. **Peak rate**: 40 Gbps, 400K flows/sec (with drops)

### Success Criteria
- **Throughput**: Sustained target rate
- **Latency**: P99 < 10ms
- **Drops**: < 0.1% packet drop rate
- **Memory**: < configured buffer size
- **CPU**: < 50% utilization

## Performance Tuning

### Configuration Parameters
- `MAX_BUFFER_SIZE_MB`: Increase for high throughput
- `BACKPRESSURE_THRESHOLD`: Tune for transport rate
- `FLOW_TIMEOUT_SECONDS`: Reduce for high flow count
- `CAPTURE_IFACE`: Optimize interface settings

### System Tuning
- Increase network interface ring buffer size
- Tune kernel network stack parameters
- Optimize CPU affinity
- Use NUMA-aware memory allocation
