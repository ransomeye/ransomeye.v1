# DPI Probe Failure Modes

**Path and File Name:** `/home/ransomeye/rebuild/ransomeye_dpi_probe/docs/failure_modes.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU  
**Details:** Failure modes, error handling, and recovery behavior documentation

---

## Failure Categories

### 1. Core Unavailability
**Symptom**: Cannot connect to Core API  
**Impact**: Events cannot be transmitted  
**Behavior**: Fail-open locally, buffer to disk  
**Recovery**: Automatic retry with exponential backoff

### 2. Backpressure
**Symptom**: Core signals backpressure  
**Impact**: Transmission rate reduced  
**Behavior**: Buffer events, reduce rate  
**Recovery**: Automatic when backpressure clears

### 3. Buffer Exhaustion
**Symptom**: Memory and disk buffers full  
**Impact**: Events dropped  
**Behavior**: Drop oldest events, log drops  
**Recovery**: Automatic when buffer space available

### 4. Signature Failure
**Symptom**: Cannot sign event  
**Impact**: Event not transmitted  
**Behavior**: Drop event, log error  
**Recovery**: Continue with next event

### 5. Identity Failure
**Symptom**: Certificate validation fails  
**Impact**: Cannot authenticate to Core  
**Behavior**: Halt transmission, log error  
**Recovery**: Manual intervention required

### 6. Resource Exhaustion
**Symptom**: CPU/Memory/Disk exhausted  
**Impact**: Performance degradation or crashes  
**Behavior**: Drop events, log warnings  
**Recovery**: Automatic cleanup, manual intervention if severe

## Failure Handling

### Fail-Open Locally
- **Principle**: Continue capturing even if Core unavailable
- **Rationale**: Don't lose network visibility
- **Implementation**: Buffer to disk, retry later
- **Risk**: Buffer exhaustion → event loss

### Fail-Closed Remotely
- **Principle**: Never send invalid events to Core
- **Rationale**: Maintain data integrity
- **Implementation**: Drop invalid events
- **Risk**: Event loss → reduced visibility

### Graceful Degradation
- **Principle**: Reduce functionality, don't crash
- **Rationale**: Maintain partial operation
- **Implementation**: Drop events when necessary
- **Risk**: Reduced visibility

## Error Recovery

### Automatic Recovery

#### Core Unavailable
1. **Detection**: Connection failure
2. **Action**: Buffer to disk
3. **Retry**: Exponential backoff (100ms → 30s)
4. **Recovery**: Resume transmission when Core available

#### Backpressure
1. **Detection**: Backpressure signal from Core
2. **Action**: Reduce transmission rate
3. **Retry**: Exponential backoff
4. **Recovery**: Resume normal rate when cleared

#### Buffer Full
1. **Detection**: Buffer at capacity
2. **Action**: Drop oldest events
3. **Retry**: Continue when space available
4. **Recovery**: Automatic (no action needed)

### Manual Recovery

#### Identity Failure
1. **Detection**: Certificate validation failure
2. **Action**: Halt transmission
3. **Recovery**: Replace certificate, restart

#### Resource Exhaustion
1. **Detection**: System resource monitoring
2. **Action**: Drop events, log warnings
3. **Recovery**: Increase resources, optimize configuration

## Failure Scenarios

### Scenario 1: Core Network Partition

**Description**: Network partition between DPI Probe and Core  
**Detection**: Connection timeouts  
**Behavior**: 
- Continue capturing packets
- Buffer events to disk
- Retry connection with exponential backoff
- Log connection failures

**Recovery**:
- Automatic when network connectivity restored
- Flush disk buffer to Core
- Resume normal operation

**Metrics**:
- Connection failure count
- Buffer utilization
- Events buffered count

### Scenario 2: Core Overload

**Description**: Core cannot process events fast enough  
**Detection**: Backpressure signals  
**Behavior**:
- Reduce transmission rate
- Buffer events in memory
- Exponential backoff retry
- Log backpressure events

**Recovery**:
- Automatic when Core ready
- Resume normal transmission rate
- Clear memory buffer

**Metrics**:
- Backpressure duration
- Buffer utilization
- Events dropped count

### Scenario 3: Disk Full

**Description**: Disk buffer directory full  
**Detection**: Write failures  
**Behavior**:
- Drop oldest events from disk buffer
- Log disk full warnings
- Continue capturing (memory buffer only)
- Monitor disk space

**Recovery**:
- Automatic when disk space available
- Manual cleanup if needed
- Increase disk space allocation

**Metrics**:
- Disk usage
- Events dropped count
- Disk write failures

### Scenario 4: Memory Exhaustion

**Description**: System memory exhausted  
**Detection**: Allocation failures  
**Behavior**:
- Drop events from memory buffer
- Reduce buffer sizes
- Log memory warnings
- Continue with reduced capacity

**Recovery**:
- Automatic cleanup of old flows
- Manual intervention if severe
- Increase system memory

**Metrics**:
- Memory usage
- Allocation failures
- Events dropped count

### Scenario 5: Signature Key Failure

**Description**: Cannot sign events (key corruption)  
**Detection**: Signing failures  
**Behavior**:
- Drop unsigned events
- Log signature errors
- Halt transmission (identity failure)
- Alert operators

**Recovery**:
- Manual: Replace certificate/key
- Restart DPI Probe
- Verify identity works

**Metrics**:
- Signature failures
- Events dropped count
- Identity validation failures

## Monitoring and Alerting

### Key Metrics
- **Connection failures**: Count of Core connection failures
- **Backpressure duration**: Time in backpressure state
- **Buffer utilization**: Current / max buffer size
- **Events dropped**: Count of dropped events
- **Signature failures**: Count of signing failures
- **Resource usage**: CPU, memory, disk usage

### Alert Thresholds
- **High connection failures**: > 10 failures/minute
- **Prolonged backpressure**: > 5 minutes
- **High buffer utilization**: > 80%
- **High drop rate**: > 100 drops/second
- **Signature failures**: > 10 failures/minute
- **Resource exhaustion**: > 90% utilization

### Alert Actions
1. **Notify operators**: Immediate notification
2. **Log details**: Detailed error logs
3. **Metrics collection**: Track failure metrics
4. **Automated recovery**: Attempt automatic recovery
5. **Escalation**: Escalate if automatic recovery fails

## Best Practices

### Prevention
- **Adequate resources**: Size buffers appropriately
- **Network redundancy**: Multiple network paths to Core
- **Certificate management**: Regular certificate rotation
- **Monitoring**: Proactive monitoring and alerting

### Detection
- **Comprehensive logging**: Log all failures
- **Metrics collection**: Track key metrics
- **Health checks**: Regular health checks
- **Alerting**: Immediate alerting on failures

### Recovery
- **Automatic recovery**: Where possible
- **Manual procedures**: Documented recovery procedures
- **Testing**: Regular failure scenario testing
- **Post-mortem**: Learn from failures
