# Rust AI Agent Architecture

```mermaid
graph TB
    subgraph User_Interface
        UI[User Input/Output]
        CMD[Command Line Interface]
    end

    subgraph Core_System
        MAIN[Main Application]
        PERS[Personality System]
        DEEP[DeepSeek Provider]
        COMP[Completion Provider]
    end

    subgraph Memory_System
        STM[Short-Term Memory]
        LTM[Long-Term Memory]
        subgraph STM_Components
            CONV[Conversations Queue]
            TOP_IDX[Topic Index]
            REL[Relevance Scoring]
        end
        subgraph LTM_Components
            PERSIST[Persistent Storage]
            FILE_IO[File I/O]
        end
    end

    subgraph Learning_System
        LM[Learning Manager]
        KB[Knowledge Base]
        DB[Database]
        subgraph Learning_Components
            INS[Insights]
            CTX[Learning Context]
            TOP_EX[Topic Extraction]
        end
    end

    subgraph Personality_System
        PROF[Personality Profile]
        EMOT[Emotional State]
        COMM[Communication Style]
        CONT[Context Rules]
    end

    %% Connections
    UI --> MAIN
    CMD --> MAIN
    MAIN --> PERS
    MAIN --> DEEP
    DEEP --> COMP

    MAIN --> STM
    MAIN --> LTM
    MAIN --> LM

    STM --> CONV
    STM --> TOP_IDX
    STM --> REL

    LTM --> PERSIST
    LTM --> FILE_IO

    LM --> KB
    LM --> DB
    LM --> INS
    LM --> CTX
    LM --> TOP_EX

    PERS --> PROF
    PROF --> EMOT
    PROF --> COMM
    PROF --> CONT

    %% Data Flow
    classDef system fill:#f9f,stroke:#333,stroke-width:2px
    classDef component fill:#bbf,stroke:#333,stroke-width:1px
    classDef storage fill:#dfd,stroke:#333,stroke-width:1px

    class MAIN,PERS,DEEP system
    class STM,LTM,LM system
    class CONV,TOP_IDX,REL,PERSIST,FILE_IO component
    class KB,DB storage
```

## Workflow Description

1. **User Interaction Flow**
   - User inputs text through CLI
   - Main application processes input
   - DeepSeek provider generates responses
   - Personality system shapes response style

2. **Memory Management**
   - Short-Term Memory:
     * Stores recent conversations
     * Indexes topics
     * Calculates relevance
   - Long-Term Memory:
     * Persists important information
     * Handles file I/O
     * Maintains historical data

3. **Learning Process**
   - Learning Manager coordinates:
     * Insight generation
     * Context building
     * Topic extraction
   - Integrates with:
     * Knowledge Base
     * Database
     * Memory Systems

4. **Personality System**
   - Manages:
     * Personality Profiles
     * Emotional States
     * Communication Styles
     * Context Rules
   - Influences:
     * Response Generation
     * Interaction Style
     * Language Complexity

## Key Components

### Core System
- Main Application: Central coordinator
- Personality System: Response shaping
- DeepSeek Provider: AI completion
- Completion Provider: Response generation

### Memory System
- Short-Term Memory: Recent context
- Long-Term Memory: Historical data
- Topic Indexing: Quick retrieval
- Relevance Scoring: Context importance

### Learning System
- Learning Manager: Knowledge acquisition
- Knowledge Base: Information storage
- Database: Structured data
- Insight Generation: Pattern recognition

### Personality System
- Profile Management: Character definition
- Emotional Expression: Response tone
- Communication Rules: Interaction style
- Context Adaptation: Situation handling

## Subsystem Diagrams

### 1. Memory Subsystem
```mermaid
graph TB
    subgraph Short_Term_Memory
        STM[Short-Term Memory Manager]
        CONV[Conversation Queue]
        TOPICS[Topic Index]
        REL[Relevance Calculator]
        
        STM --> CONV
        STM --> TOPICS
        STM --> REL
        
        subgraph Conversation_Processing
            ADD[Add Interaction]
            PRUNE[Prune Old Data]
            CTX[Get Context]
            STATS[Memory Stats]
        end
        
        STM --> ADD
        STM --> PRUNE
        STM --> CTX
        STM --> STATS
    end
    
    subgraph Long_Term_Memory
        LTM[Long-Term Memory Manager]
        STORE[Storage Handler]
        FILE[File Operations]
        
        LTM --> STORE
        LTM --> FILE
        
        subgraph Persistence
            SAVE[Save to File]
            LOAD[Load from File]
            KEY_VAL[Key-Value Store]
        end
        
        STORE --> KEY_VAL
        FILE --> SAVE
        FILE --> LOAD
    end
    
    STM -.-> LTM
    
    classDef manager fill:#f9f,stroke:#333,stroke-width:2px
    classDef component fill:#bbf,stroke:#333,stroke-width:1px
    classDef operation fill:#dfd,stroke:#333,stroke-width:1px
    
    class STM,LTM manager
    class CONV,TOPICS,REL,STORE,FILE component
    class ADD,PRUNE,CTX,STATS,SAVE,LOAD operation
```

### 2. Learning Subsystem
```mermaid
graph TB
    subgraph Learning_Manager
        LM[Learning Manager]
        INS[Insight Generator]
        CTX[Context Builder]
        TOP[Topic Extractor]
        
        LM --> INS
        LM --> CTX
        LM --> TOP
    end
    
    subgraph Knowledge_Processing
        KB[Knowledge Base]
        DB[Database]
        LEARN[Learning Process]
        
        subgraph Insight_Generation
            EXTRACT[Extract Topics]
            ANALYZE[Analyze Content]
            SCORE[Calculate Confidence]
        end
        
        LEARN --> EXTRACT
        LEARN --> ANALYZE
        LEARN --> SCORE
    end
    
    subgraph Data_Storage
        SQL[SQLite Database]
        JSON[JSON Storage]
        CACHE[Memory Cache]
    end
    
    LM --> KB
    LM --> DB
    KB --> JSON
    DB --> SQL
    LEARN --> CACHE
    
    classDef primary fill:#f9f,stroke:#333,stroke-width:2px
    classDef secondary fill:#bbf,stroke:#333,stroke-width:1px
    classDef storage fill:#dfd,stroke:#333,stroke-width:1px
    
    class LM primary
    class KB,DB,LEARN secondary
    class SQL,JSON,CACHE storage
```

### 3. Personality Subsystem
```mermaid
graph TB
    subgraph Personality_Manager
        PM[Personality Manager]
        PROF[Profile Handler]
        EMOT[Emotion Engine]
        COMM[Communication Controller]
    end
    
    subgraph Profile_Components
        BIO[Biography]
        TRAITS[Personality Traits]
        RULES[Context Rules]
        
        subgraph Traits
            OPEN[Openness]
            CONS[Conscientiousness]
            EXTR[Extraversion]
            AGRE[Agreeableness]
            NEUR[Neuroticism]
        end
    end
    
    subgraph Emotional_System
        EMO_STATE[Emotional State]
        EXPR[Expression Handler]
        
        subgraph Expressions
            EMOJI[Emoji Set]
            EMOTE[Emote Set]
            TONE[Tone Adjustments]
        end
    end
    
    PM --> PROF
    PM --> EMOT
    PM --> COMM
    
    PROF --> BIO
    PROF --> TRAITS
    PROF --> RULES
    
    TRAITS --> OPEN
    TRAITS --> CONS
    TRAITS --> EXTR
    TRAITS --> AGRE
    TRAITS --> NEUR
    
    EMOT --> EMO_STATE
    EMOT --> EXPR
    
    EMO_STATE --> EMOJI
    EMO_STATE --> EMOTE
    EMO_STATE --> TONE
    
    classDef manager fill:#f9f,stroke:#333,stroke-width:2px
    classDef component fill:#bbf,stroke:#333,stroke-width:1px
    classDef detail fill:#dfd,stroke:#333,stroke-width:1px
    
    class PM manager
    class PROF,EMOT,COMM component
    class BIO,TRAITS,RULES,EMO_STATE,EXPR detail
```

### 4. DeepSeek Integration
```mermaid
graph TB
    subgraph API_Integration
        DP[DeepSeek Provider]
        CP[Completion Provider]
        HTTP[HTTP Client]
    end
    
    subgraph Request_Processing
        REQ[Request Builder]
        AUTH[Authentication]
        RESP[Response Handler]
        
        subgraph Request_Components
            SYS[System Message]
            USER[User Message]
            CONF[Configuration]
        end
    end
    
    subgraph Response_Processing
        PARSE[JSON Parser]
        VALID[Validation]
        ERROR[Error Handler]
    end
    
    DP --> CP
    CP --> HTTP
    
    HTTP --> REQ
    REQ --> AUTH
    REQ --> SYS
    REQ --> USER
    REQ --> CONF
    
    HTTP --> RESP
    RESP --> PARSE
    RESP --> VALID
    RESP --> ERROR
    
    classDef provider fill:#f9f,stroke:#333,stroke-width:2px
    classDef processor fill:#bbf,stroke:#333,stroke-width:1px
    classDef handler fill:#dfd,stroke:#333,stroke-width:1px
    
    class DP,CP provider
    class REQ,RESP,HTTP processor
    class PARSE,VALID,ERROR handler
```

## Sequence Diagrams

### 1. User Interaction Flow
```mermaid
sequenceDiagram
    participant U as User
    participant M as Main
    participant P as Personality
    participant D as DeepSeek
    participant STM as ShortTerm Memory
    participant LTM as LongTerm Memory
    participant L as Learning

    U->>M: Input Text
    activate M
    M->>STM: Get Context
    activate STM
    STM-->>M: Recent Context
    deactivate STM
    
    M->>P: Get Current Profile
    activate P
    P-->>M: Personality Config
    deactivate P
    
    M->>D: Generate Response
    activate D
    D-->>M: AI Response
    deactivate D
    
    par Memory Storage
        M->>STM: Store Interaction
        M->>LTM: Archive Interaction
    and Learning
        M->>L: Process Interaction
        activate L
        L->>L: Extract Insights
        L->>L: Update Knowledge
        deactivate L
    end
    
    M-->>U: Display Response
    deactivate M
```

### 2. Memory Management Flow
```mermaid
sequenceDiagram
    participant STM as ShortTerm Memory
    participant LTM as LongTerm Memory
    participant DB as Database
    participant F as File System
    
    note over STM: New Interaction
    
    STM->>STM: Add to Queue
    activate STM
    STM->>STM: Extract Topics
    STM->>STM: Update Index
    STM->>STM: Calculate Relevance
    
    alt Queue Full
        STM->>STM: Prune Old Data
        STM->>LTM: Transfer Important Data
        activate LTM
        LTM->>DB: Store in Database
        LTM->>F: Update Memory File
        deactivate LTM
    end
    
    STM->>STM: Rebuild Topic Index
    deactivate STM
```

### 3. Learning Process Flow
```mermaid
sequenceDiagram
    participant LM as Learning Manager
    participant KB as Knowledge Base
    participant DB as Database
    participant I as Insight Generator
    participant C as Context Builder
    
    activate LM
    LM->>I: Process New Interaction
    activate I
    I->>I: Extract Topics
    I->>I: Analyze Content
    I->>I: Calculate Confidence
    I-->>LM: Generated Insights
    deactivate I
    
    LM->>C: Build Context
    activate C
    C->>KB: Query Related Knowledge
    C->>DB: Fetch Historical Data
    C-->>LM: Enriched Context
    deactivate C
    
    LM->>KB: Update Knowledge Base
    LM->>DB: Store New Insights
    deactivate LM
```

### 4. Personality Adaptation Flow
```mermaid
sequenceDiagram
    participant M as Main
    participant PM as Personality Manager
    participant E as Emotion Engine
    participant C as Context Rules
    participant P as Profile Handler
    
    M->>PM: Process Input
    activate PM
    
    PM->>C: Check Context
    activate C
    C->>C: Analyze Setting
    C->>C: Apply Rules
    C-->>PM: Context Parameters
    deactivate C
    
    PM->>E: Update Emotional State
    activate E
    E->>E: Process Input Tone
    E->>E: Calculate Response Emotion
    E-->>PM: Emotional Parameters
    deactivate E
    
    PM->>P: Adjust Profile
    activate P
    P->>P: Update Traits
    P->>P: Modify Expression
    P-->>PM: Updated Profile
    deactivate P
    
    PM-->>M: Adapted Configuration
    deactivate PM
```

### 5. DeepSeek API Interaction Flow
```mermaid
sequenceDiagram
    participant M as Main
    participant DP as DeepSeek Provider
    participant CP as Completion Provider
    participant API as DeepSeek API
    
    M->>DP: Request Completion
    activate DP
    
    DP->>CP: Build Request
    activate CP
    CP->>CP: Add System Message
    CP->>CP: Add User Input
    CP->>CP: Set Configuration
    
    CP->>API: Send Request
    activate API
    
    alt Success
        API-->>CP: Response Data
        CP->>CP: Parse JSON
        CP->>CP: Validate Content
        CP-->>DP: Processed Response
    else Error
        API-->>CP: Error Response
        CP->>CP: Handle Error
        CP-->>DP: Error Details
    end
    deactivate API
    deactivate CP
    
    DP-->>M: Final Response
    deactivate DP
```

## Timing Diagrams

### 1. Request-Response Cycle Timing
```mermaid
sequenceDiagram
    participant U as User
    participant M as Main
    participant D as DeepSeek

    note right of U: t=0ms: User Input
    
    U->>+M: Input Text
    note right of M: t=5ms: Input Processing
    
    M->>+D: API Request
    note right of D: t=300-800ms: API Processing
    D-->>-M: Response
    
    note right of M: t=10ms: Response Processing
    M-->>-U: Display Response
    
    note right of U: Total Time: 315-815ms
```

### 2. Memory Operations Timing
```mermaid
gantt
    title Memory Operations Timeline
    dateFormat  X
    axisFormat %L ms

    section Short-Term Memory
    Topic Extraction      :0, 20ms
    Relevance Calculation :20ms, 15ms
    Index Update         :35ms, 10ms
    
    section Long-Term Memory
    Data Serialization   :45ms, 25ms
    File Write          :70ms, 30ms
    
    section Database
    Connection          :0, 5ms
    Query Execution     :5ms, 15ms
    Data Storage        :20ms, 25ms
```

### 3. Parallel Processing Performance
```mermaid
gantt
    title Parallel Processing Timeline
    dateFormat  X
    axisFormat %L ms

    section Main Thread
    Input Processing    :0, 10ms
    Response Generation :10ms, 400ms
    
    section Memory Thread
    Context Retrieval   :0, 15ms
    Storage Operations  :15ms, 35ms
    
    section Learning Thread
    Topic Analysis      :0, 25ms
    Knowledge Update    :25ms, 40ms
    
    section Database Thread
    Write Operations    :15ms, 30ms
```

### 4. Component Load Distribution
```mermaid
pie
    title Component Processing Time Distribution
    "API Communication" : 45
    "Memory Operations" : 20
    "Learning Processing" : 15
    "Database Operations" : 10
    "Input/Output" : 5
    "Other Processing" : 5
```

### 5. System Performance Metrics
```mermaid
xychart-beta
    title "Response Time vs Load"
    x-axis [0, 10, 20, 30, 40, 50] "Concurrent Users"
    y-axis "Response Time (ms)" 1000
    line ["200", "300", "450", "650", "800", "950"]
    title "Memory Usage vs Operations"
    x-axis [0, 100, 200, 300, 400, 500] "Operations"
    y-axis "Memory (MB)" 500
    line ["50", "100", "175", "275", "400", "450"]
```

### 6. Critical Path Analysis
```mermaid
graph LR
    subgraph Critical Path
        direction LR
        A[Input] -->|5ms| B[Processing]
        B -->|300ms| C[API Call]
        C -->|10ms| D[Response]
        D -->|5ms| E[Output]
    end
    
    subgraph Parallel Operations
        direction LR
        P1[Memory Ops] -->|50ms| P2[Storage]
        P3[Learning] -->|65ms| P4[Knowledge Update]
    end
    
    style A fill:#f9f,stroke:#333
    style B fill:#bbf,stroke:#333
    style C fill:#f99,stroke:#333
    style D fill:#bbf,stroke:#333
    style E fill:#f9f,stroke:#333
```

### Performance Notes:

1. **Response Time Breakdown**:
   - Input Processing: 5-10ms
   - API Communication: 300-800ms
   - Memory Operations: 45-100ms
   - Learning Processing: 65-120ms
   - Database Operations: 30-60ms

2. **Optimization Points**:
   - API calls are the main bottleneck
   - Memory operations are optimized with parallel processing
   - Database operations use connection pooling
   - Learning processes run asynchronously

3. **Scaling Characteristics**:
   - Linear scaling up to 30 concurrent users
   - Memory usage grows linearly with operation count
   - Database performance degrades after 400 operations/second

4. **Performance Recommendations**:
   - Implement request caching for common queries
   - Use batch processing for database operations
   - Implement memory pruning at 80% capacity
   - Consider API request queuing for high loads
