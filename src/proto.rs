// follow ql2.proto
// ----------------------------------------------------------------------------------------
// NOTE: these must impl the COPY trait if casting is needed in a borrowed context.
// Otherwise, casting to the numeric value constitutes a move and jams up the ast type
// that made use of the enum. Not really a big deal, since they're all basically ints that
// should be copyable.
// ----------------------------------------------------------------------------------------

pub enum Version {
    V0_1 = 0x3f61ba36,
    V0_2 = 0x723081e1, // Authorization key during handshake
    V0_3 = 0x5f75e83e, // Authorization key and protocol during handshake
    V0_4 = 0x400c2d20
}


pub enum Protocol {
    PROTOBUF = 0x271ffc41,
    JSON     = 0x7e6970c7
}

#[derive(Copy, Clone)]
pub enum QueryType {
    START         = 1,
    CONTINUE      = 2,
    STOP          = 3,
    NOREPLY_WAIT  = 4,
    SERVER_INFO   = 5
}


pub enum FrameType {
    POS = 1, // Error occured in a positional argument.
    OPT = 2  // Error occured in an optional argument.
}


pub enum ResponseType {
    // These response types indicate success.
    SUCCESS_ATOM      = 1, // Query returned a single RQL datatype.
    SUCCESS_SEQUENCE  = 2, // Query returned a sequence of RQL datatypes.
    SUCCESS_PARTIAL   = 3, // Query returned a partial sequence of RQL
                           // datatypes.  If you send a [CONTINUE] query with
                           // the same token as this response, you will get
                           // more of the sequence.  Keep sending [CONTINUE]
                           // queries until you get back [SUCCESS_SEQUENCE].
    WAIT_COMPLETE     = 4, // A [NOREPLY_WAIT] query completed.
    SERVER_INFO       = 5, // The data for a [SERVER_INFO] request.  This is
                           // the same as `SUCCESS_ATOM` except that there will
                           // never be profiling data.

    // These response types indicate failure.
    CLIENT_ERROR      = 16, // Means the client is buggy.  An example is if the
                            // client sends a malformed protobuf, or tries to
                            // send [CONTINUE] for an unknown token.
    COMPILE_ERROR     = 17, // Means the query failed during parsing or type
                            // checking.  For example, if you pass too many
                            // arguments to a function.
    RUNTIME_ERROR     = 18  // Means the query failed at runtime.  An example is
                            // if you add together two values from a table, but
                            // they turn out at runtime to be booleans rather
                            // than numbers.
}


// If `ResponseType` is `RUNTIME_ERROR`, this may be filled in with more
// information about the error.
pub enum ErrorType {
    INTERNAL         = 1000000,
    RESOURCE_LIMIT   = 2000000,
    QUERY_LOGIC      = 3000000,
    NON_EXISTENCE    = 3100000,
    OP_FAILED        = 4100000,
    OP_INDETERMINATE = 4200000,
    USER             = 5000000
}


pub enum ResponseNote {
    // The stream is a changefeed stream (e.g. `r.table('test').changes()`).
    SEQUENCE_FEED = 1,
    // The stream is a point changefeed stream
    // (e.g. `r.table('test').get(0).changes()`).
    ATOM_FEED = 2,
    // The stream is an order_by_limit changefeed stream
    // (e.g. `r.table('test').order_by(index: 'id').limit(5).changes()`).
    ORDER_BY_LIMIT_FEED = 3,
    // The stream is a union of multiple changefeed types that can't be
    // collapsed to a single type
    // (e.g. `r.table('test').changes().union(r.table('test').get(0).changes())`).
    UNIONED_FEED = 4,
    // The stream is a changefeed stream and includes notes on what state
    // the changefeed stream is in (e.g. objects of the form `{state:
    // 'initializing'}`).
    INCLUDES_STATES = 5
}

pub enum DatumType {
    R_NULL   = 1,
    R_BOOL   = 2,
    R_NUM    = 3, // a double
    R_STR    = 4,
    R_ARRAY  = 5,
    R_OBJECT = 6,
    // This [DatumType] will only be used if [accepts_r_json] is
    // set to [true] in [Query].  [r_str] will be filled with a
    // JSON encoding of the [Datum].
    R_JSON   = 7 // uses r_str
}

#[derive(Copy, Clone)]
pub enum TermType {
    // A RQL datum, stored in `datum` below.
    DATUM = 1,

    MAKE_ARRAY = 2, // DATUM... -> ARRAY
    // Evaluate the terms in [optargs] and make an object
    MAKE_OBJ   = 3, // {...} -> OBJECT

    // * Compound types

    // Takes an integer representing a variable and returns the value stored
    // in that variable.  It's the responsibility of the client to translate
    // from their local representation of a variable to a unique _non-negative_
    // integer for that variable.  (We do it this way instead of letting
    // clients provide variable names as strings to discourage
    // variable-capturing client libraries, and because it's more efficient
    // on the wire.)
    VAR          = 10, // !NUMBER -> DATUM
    // Takes some javascript code and executes it.
    JAVASCRIPT   = 11, // STRING {timeout: !NUMBER} -> DATUM |
                   // STRING {timeout: !NUMBER} -> Function(*)
    UUID = 169, // () -> DATUM

    // Takes an HTTP URL and gets it.  If the get succeeds and
    //  returns valid JSON, it is converted into a DATUM
    HTTP = 153, // STRING {data: OBJECT | STRING,
            //         timeout: !NUMBER,
            //         method: STRING,
            //         params: OBJECT,
            //         header: OBJECT | ARRAY,
            //         attempts: NUMBER,
            //         redirects: NUMBER,
            //         verify: BOOL,
            //         page: FUNC | STRING,
            //         page_limit: NUMBER,
            //         auth: OBJECT,
            //         result_format: STRING,
            //         } -> STRING | STREAM

    // Takes a string and throws an error with that message.
    // Inside of a `default` block, you can omit the first
    // argument to rethrow whatever error you catch (this is most
    // useful as an argument to the `default` filter optarg).
    ERROR        = 12, // STRING -> Error | -> Error
    // Takes nothing and returns a reference to the implicit variable.
    IMPLICIT_VAR = 13, // -> DATUM

    // * Data Operators
    // Returns a reference to a database.
    DB    = 14, // STRING -> Database
    // Returns a reference to a table.
    TABLE = 15, // Database, STRING, {read_mode:STRING, identifier_format:STRING} -> Table
            // STRING, {read_mode:STRING, identifier_format:STRING} -> Table
    // Gets a single element from a table by its primary or a secondary key.
    GET   = 16, // Table, STRING -> SingleSelection | Table, NUMBER -> SingleSelection |
            // Table, STRING -> NULL            | Table, NUMBER -> NULL |
    GET_ALL = 78, // Table, DATUM..., {index:!STRING} => ARRAY

    // Simple DATUM Ops
    EQ  = 17, // DATUM... -> BOOL
    NE  = 18, // DATUM... -> BOOL
    LT  = 19, // DATUM... -> BOOL
    LE  = 20, // DATUM... -> BOOL
    GT  = 21, // DATUM... -> BOOL
    GE  = 22, // DATUM... -> BOOL
    NOT = 23, // BOOL -> BOOL
    // ADD can either add two numbers or concatenate two arrays.
    ADD = 24, // NUMBER... -> NUMBER | STRING... -> STRING
    SUB = 25, // NUMBER... -> NUMBER
    MUL = 26, // NUMBER... -> NUMBER
    DIV = 27, // NUMBER... -> NUMBER
    MOD = 28, // NUMBER, NUMBER -> NUMBER

    FLOOR = 183,    // NUMBER -> NUMBER
    CEIL = 184,     // NUMBER -> NUMBER
    ROUND = 185,    // NUMBER -> NUMBER

    // DATUM Array Ops
    // Append a single element to the end of an array (like `snoc`).
    APPEND = 29, // ARRAY, DATUM -> ARRAY
    // Prepend a single element to the end of an array (like `cons`).
    PREPEND = 80, // ARRAY, DATUM -> ARRAY
    //Remove the elements of one array from another array.
    DIFFERENCE = 95, // ARRAY, ARRAY -> ARRAY

    // DATUM Set Ops
    // Set ops work on arrays. They don't use actual sets and thus have
    // performance characteristics you would expect from arrays rather than
    // from sets. All set operations have the post condition that they
    // array they return contains no duplicate values.
    SET_INSERT = 88, // ARRAY, DATUM -> ARRAY
    SET_INTERSECTION = 89, // ARRAY, ARRAY -> ARRAY
    SET_UNION = 90, // ARRAY, ARRAY -> ARRAY
    SET_DIFFERENCE = 91, // ARRAY, ARRAY -> ARRAY

    SLICE  = 30, // Sequence, NUMBER, NUMBER -> Sequence
    SKIP  = 70, // Sequence, NUMBER -> Sequence
    LIMIT = 71, // Sequence, NUMBER -> Sequence
    OFFSETS_OF = 87, // Sequence, DATUM -> Sequence | Sequence, Function(1) -> Sequence
    CONTAINS = 93, // Sequence, (DATUM | Function(1))... -> BOOL

    // Stream/Object Ops
    // Get a particular field from an object, or map that over a
    // sequence.
    GET_FIELD  = 31, // OBJECT, STRING -> DATUM
                 // | Sequence, STRING -> Sequence
    // Return an array containing the keys of the object.
    KEYS = 94, // OBJECT -> ARRAY
    // Return an array containing the values of the object.
    VALUES = 186, // OBJECT -> ARRAY
    // Creates an object
    OBJECT = 143, // STRING, DATUM, ... -> OBJECT
    // Check whether an object contains all the specified fields,
    // or filters a sequence so that all objects inside of it
    // contain all the specified fields.
    HAS_FIELDS = 32, // OBJECT, Pathspec... -> BOOL
    // x.with_fields(...) <=> x.has_fields(...).pluck(...)
    WITH_FIELDS = 96, // Sequence, Pathspec... -> Sequence
    // Get a subset of an object by selecting some attributes to preserve,
    // or map that over a sequence.  (Both pick and pluck, polymorphic.)
    PLUCK    = 33, // Sequence, Pathspec... -> Sequence | OBJECT, Pathspec... -> OBJECT
    // Get a subset of an object by selecting some attributes to discard, or
    // map that over a sequence.  (Both unpick and without, polymorphic.)
    WITHOUT  = 34, // Sequence, Pathspec... -> Sequence | OBJECT, Pathspec... -> OBJECT
    // Merge objects (right-preferential)
    MERGE    = 35, // OBJECT... -> OBJECT | Sequence -> Sequence

    // Sequence Ops
    // Get all elements of a sequence between two values.
    // Half-open by default, but the openness of either side can be
    // changed by passing 'closed' or 'open for `right_bound` or
    // `left_bound`.
    BETWEEN_DEPRECATED = 36, // Deprecated version of between, which allows `null` to specify unboundedness
                         // With the newer version, clients should use `r.minval` and `r.maxval` for unboundedness
    BETWEEN   = 182, // StreamSelection, DATUM, DATUM, {index:!STRING, right_bound:STRING, left_bound:STRING} -> StreamSelection
    REDUCE    = 37, // Sequence, Function(2) -> DATUM
    MAP       = 38, // Sequence, Function(1) -> Sequence
                // The arity of the function should be
                // Sequence..., Function(sizeof...(Sequence)) -> Sequence

    // Filter a sequence with either a function or a shortcut
    // object (see API docs for details).  The body of FILTER is
    // wrapped in an implicit `.default(false)`, and you can
    // change the default value by specifying the `default`
    // optarg.  If you make the default `r.error`, all errors
    // caught by `default` will be rethrown as if the `default`
    // did not exist.
    FILTER    = 39, // Sequence, Function(1), {default:DATUM} -> Sequence |
                // Sequence, OBJECT, {default:DATUM} -> Sequence
    // Map a function over a sequence and then concatenate the results together.
    CONCAT_MAP = 40, // Sequence, Function(1) -> Sequence
    // Order a sequence based on one or more attributes.
    ORDER_BY   = 41, // Sequence, (!STRING | Ordering)..., {index: (!STRING | Ordering)} -> Sequence
    // Get all distinct elements of a sequence (like `uniq`).
    DISTINCT  = 42, // Sequence -> Sequence
    // Count the number of elements in a sequence, or only the elements that match
    // a given filter.
    COUNT     = 43, // Sequence -> NUMBER | Sequence, DATUM -> NUMBER | Sequence, Function(1) -> NUMBER
    IS_EMPTY = 86, // Sequence -> BOOL
    // Take the union of multiple sequences (preserves duplicate elements! (use distinct)).
    UNION     = 44, // Sequence... -> Sequence
    // Get the Nth element of a sequence.
    NTH       = 45, // Sequence, NUMBER -> DATUM
    // do NTH or GET_FIELD depending on target object
    BRACKET            = 170, // Sequence | OBJECT, NUMBER | STRING -> DATUM
    // OBSOLETE_GROUPED_MAPREDUCE = 46,
    // OBSOLETE_GROUPBY = 47,

    INNER_JOIN         = 48, // Sequence, Sequence, Function(2) -> Sequence
    OUTER_JOIN         = 49, // Sequence, Sequence, Function(2) -> Sequence
    // An inner-join that does an equality comparison on two attributes.
    EQ_JOIN            = 50, // Sequence, !STRING, Sequence, {index:!STRING} -> Sequence
    ZIP                = 72, // Sequence -> Sequence
    RANGE              = 173, // -> Sequence                        [0, +inf)
                          // NUMBER -> Sequence                 [0, a)
                          // NUMBER, NUMBER -> Sequence         [a, b)

    // Array Ops
    // Insert an element in to an array at a given index.
    INSERT_AT          = 82, // ARRAY, NUMBER, DATUM -> ARRAY
    // Remove an element at a given index from an array.
    DELETE_AT          = 83, // ARRAY, NUMBER -> ARRAY |
                         // ARRAY, NUMBER, NUMBER -> ARRAY
    // Change the element at a given index of an array.
    CHANGE_AT          = 84, // ARRAY, NUMBER, DATUM -> ARRAY
    // Splice one array in to another array.
    SPLICE_AT          = 85, // ARRAY, NUMBER, ARRAY -> ARRAY

    // * Type Ops
    // Coerces a datum to a named type (e.g. "bool").
    // If you previously used `stream_to_array`, you should use this instead
    // with the type "array".
    COERCE_TO = 51, // Top, STRING -> Top
    // Returns the named type of a datum (e.g. TYPE_OF(true) = "BOOL")
    TYPE_OF = 52, // Top -> STRING

    // * Write Ops (the OBJECTs contain data about number of errors etc.)
    // Updates all the rows in a selection.  Calls its Function with the row
    // to be updated, and then merges the result of that call.
    UPDATE   = 53, // StreamSelection, Function(1), {non_atomic:BOOL, durability:STRING, return_changes:BOOL} -> OBJECT |
               // SingleSelection, Function(1), {non_atomic:BOOL, durability:STRING, return_changes:BOOL} -> OBJECT |
               // StreamSelection, OBJECT,      {non_atomic:BOOL, durability:STRING, return_changes:BOOL} -> OBJECT |
               // SingleSelection, OBJECT,      {non_atomic:BOOL, durability:STRING, return_changes:BOOL} -> OBJECT
    // Deletes all the rows in a selection.
    DELETE   = 54, // StreamSelection, {durability:STRING, return_changes:BOOL} -> OBJECT | SingleSelection -> OBJECT
    // Replaces all the rows in a selection.  Calls its Function with the row
    // to be replaced, and then discards it and stores the result of that
    // call.
    REPLACE  = 55, // StreamSelection, Function(1), {non_atomic:BOOL, durability:STRING, return_changes:BOOL} -> OBJECT | SingleSelection, Function(1), {non_atomic:BOOL, durability:STRING, return_changes:BOOL} -> OBJECT
    // Inserts into a table.  If `conflict` is replace, overwrites
    // entries with the same primary key.  If `conflict` is
    // update, does an update on the entry.  If `conflict` is
    // error, or is omitted, conflicts will trigger an error.
    INSERT   = 56, // Table, OBJECT, {conflict:STRING, durability:STRING, return_changes:BOOL} -> OBJECT | Table, Sequence, {conflict:STRING, durability:STRING, return_changes:BOOL} -> OBJECT

    // * Administrative OPs
    // Creates a database with a particular name.
    DB_CREATE     = 57, // STRING -> OBJECT
    // Drops a database with a particular name.
    DB_DROP       = 58, // STRING -> OBJECT
    // Lists all the databases by name.  (Takes no arguments)
    DB_LIST       = 59, // -> ARRAY
    // Creates a table with a particular name in a particular
    // database.  (You may omit the first argument to use the
    // default database.)
    TABLE_CREATE  = 60, // Database, STRING, {primary_key:STRING, shards:NUMBER, replicas:NUMBER, primary_replica_tag:STRING} -> OBJECT
                    // Database, STRING, {primary_key:STRING, shards:NUMBER, replicas:OBJECT, primary_replica_tag:STRING} -> OBJECT
                    // STRING, {primary_key:STRING, shards:NUMBER, replicas:NUMBER, primary_replica_tag:STRING} -> OBJECT
                    // STRING, {primary_key:STRING, shards:NUMBER, replicas:OBJECT, primary_replica_tag:STRING} -> OBJECT
    // Drops a table with a particular name from a particular
    // database.  (You may omit the first argument to use the
    // default database.)
    TABLE_DROP    = 61, // Database, STRING -> OBJECT
                        // STRING -> OBJECT
    // Lists all the tables in a particular database.  (You may
    // omit the first argument to use the default database.)
    TABLE_LIST    = 62, // Database -> ARRAY
                    //  -> ARRAY
    // Returns the row in the `rethinkdb.table_config` or `rethinkdb.db_config` table
    // that corresponds to the given database or table.
    CONFIG  = 174, // Database -> SingleSelection
               // Table -> SingleSelection
    // Returns the row in the `rethinkdb.table_status` table that corresponds to the
    // given table.
    STATUS  = 175, // Table -> SingleSelection
    // Called on a table, waits for that table to be ready for read/write operations.
    // Called on a database, waits for all of the tables in the database to be ready.
    // Returns the corresponding row or rows from the `rethinkdb.table_status` table.
    WAIT    = 177, // Table -> OBJECT
               // Database -> OBJECT
    // Generates a new config for the given table, or all tables in the given database
    // The `shards` and `replicas` arguments are required. If `emergency_repair` is
    // specified, it will enter a completely different mode of repairing a table
    // which has lost half or more of its replicas.
    RECONFIGURE   = 176, // Database|Table, {shards:NUMBER, replicas:NUMBER [,
                     //                  dry_run:BOOLEAN]
                     //                 } -> OBJECT
                     // Database|Table, {shards:NUMBER, replicas:OBJECT [,
                     //                  primary_replica_tag:STRING,
                     //                  nonvoting_replica_tags:ARRAY,
                     //                  dry_run:BOOLEAN]
                     //                 } -> OBJECT
                     // Table, {emergency_repair:STRING, dry_run:BOOLEAN} -> OBJECT
    // Balances the table's shards but leaves everything else the same. Can also be
    // applied to an entire database at once.
    REBALANCE     = 179, // Table -> OBJECT
                     // Database -> OBJECT

    // Ensures that previously issued soft-durability writes are complete and
    // written to disk.
    SYNC          = 138, // Table -> OBJECT

    // * Secondary indexes OPs
    // Creates a new secondary index with a particular name and definition.
    INDEX_CREATE = 75, // Table, STRING, Function(1), {multi:BOOL} -> OBJECT
    // Drops a secondary index with a particular name from the specified table.
    INDEX_DROP   = 76, // Table, STRING -> OBJECT
    // Lists all secondary indexes on a particular table.
    INDEX_LIST   = 77, // Table -> ARRAY
    // Gets information about whether or not a set of indexes are ready to
    // be accessed. Returns a list of objects that look like this:
    // {index:STRING, ready:BOOL[, progress:NUMBER]}
    INDEX_STATUS = 139, // Table, STRING... -> ARRAY
    // Blocks until a set of indexes are ready to be accessed. Returns the
    // same values INDEX_STATUS.
    INDEX_WAIT = 140, // Table, STRING... -> ARRAY
    // Renames the given index to a new name
    INDEX_RENAME = 156, // Table, STRING, STRING, {overwrite:BOOL} -> OBJECT

    // * Control Operators
    // Calls a function on data
    FUNCALL  = 64, // Function(*), DATUM... -> DATUM
    // Executes its first argument, and returns its second argument if it
    // got [true] or its third argument if it got [false] (like an `if`
    // statement).
    BRANCH  = 65, // BOOL, Top, Top -> Top
    // Returns true if any of its arguments returns true (short-circuits).
    OR      = 66, // BOOL... -> BOOL
    // Returns true if all of its arguments return true (short-circuits).
    AND     = 67, // BOOL... -> BOOL
    // Calls its Function with each entry in the sequence
    // and executes the array of terms that Function returns.
    FOR_EACH = 68, // Sequence, Function(1) -> OBJECT

    ////////////////////////////////////////////////////////////////////////////////
    ////////// Special Terms
    ////////////////////////////////////////////////////////////////////////////////

    // An anonymous function.  Takes an array of numbers representing
    // variables (see [VAR] above), and a [Term] to execute with those in
    // scope.  Returns a function that may be passed an array of arguments,
    // then executes the Term with those bound to the variable names.  The
    // user will never construct this directly.  We use it internally for
    // things like `map` which take a function.  The "arity" of a [Function] is
    // the number of arguments it takes.
    // For example, here's what `_X_.map{|x| x+2}` turns into:
    // Term {
    //   type = MAP,
    //   args = [_X_,
    //           Term {
    //             type = Function,
    //             args = [Term {
    //                       type = DATUM,
    //                       datum = Datum {
    //                         type = R_ARRAY,
    //                         r_array = [Datum { type = R_NUM, r_num = 1, }],
    //                       },
    //                     },
    //                     Term {
    //                       type = ADD,
    //                       args = [Term {
    //                                 type = VAR,
    //                                 args = [Term {
    //                                           type = DATUM,
    //                                           datum = Datum { type = R_NUM,
    //                                                           r_num = 1},
    //                                         }],
    //                               },
    //                               Term {
    //                                 type = DATUM,
    //                                 datum = Datum { type = R_NUM, r_num = 2, },
    //                               }],
    //                     }],
    //           }],
    FUNC = 69, // ARRAY, Top -> ARRAY -> Top

    // Indicates to ORDER_BY that this attribute is to be sorted in ascending order.
    ASC = 73, // !STRING -> Ordering
    // Indicates to ORDER_BY that this attribute is to be sorted in descending order.
    DESC = 74, // !STRING -> Ordering

    // Gets info about anything.  INFO is most commonly called on tables.
    INFO = 79, // Top -> OBJECT

    // `a.match(b)` returns a match object if the string `a`
    // matches the regular expression `b`.
    MATCH = 97, // STRING, STRING -> DATUM

    // Change the case of a string.
    UPCASE   = 141, // STRING -> STRING
    DOWNCASE = 142, // STRING -> STRING

    // Select a number of elements from sequence with uniform distribution.
    SAMPLE = 81, // Sequence, NUMBER -> Sequence

    // Evaluates its first argument.  If that argument returns
    // NULL or throws an error related to the absence of an
    // expected value (for instance, accessing a non-existent
    // field or adding NULL to an integer), DEFAULT will either
    // return its second argument or execute it if it's a
    // function.  If the second argument is a function, it will be
    // passed either the text of the error or NULL as its
    // argument.
    DEFAULT = 92, // Top, Top -> Top

    // Parses its first argument as a json string and returns it as a
    // datum.
    JSON = 98, // STRING -> DATUM
    // Returns the datum as a JSON string.
    // N.B.: we would really prefer this be named TO_JSON and that exists as
    // an alias in Python and JavaScript drivers, however it conflicts with the
    // standard `to_json` method defined by Ruby's standard json library.
    TO_JSON_STRING = 172, // DATUM -> STRING

    // Parses its first arguments as an ISO 8601 time and returns it as a
    // datum.
    ISO8601 = 99, // STRING -> PSEUDOTYPE(TIME)
    // Prints a time as an ISO 8601 time.
    TO_ISO8601 = 100, // PSEUDOTYPE(TIME) -> STRING

    // Returns a time given seconds since epoch in UTC.
    EPOCH_TIME = 101, // NUMBER -> PSEUDOTYPE(TIME)
    // Returns seconds since epoch in UTC given a time.
    TO_EPOCH_TIME = 102, // PSEUDOTYPE(TIME) -> NUMBER

    // The time the query was received by the server.
    NOW = 103, // -> PSEUDOTYPE(TIME)
    // Puts a time into an ISO 8601 timezone.
    IN_TIMEZONE = 104, // PSEUDOTYPE(TIME), STRING -> PSEUDOTYPE(TIME)
    // a.during(b, c) returns whether a is in the range [b, c)
    DURING = 105, // PSEUDOTYPE(TIME), PSEUDOTYPE(TIME), PSEUDOTYPE(TIME) -> BOOL
    // Retrieves the date portion of a time.
    DATE = 106, // PSEUDOTYPE(TIME) -> PSEUDOTYPE(TIME)
    // x.time_of_day == x.date - x
    TIME_OF_DAY = 126, // PSEUDOTYPE(TIME) -> NUMBER
    // Returns the timezone of a time.
    TIMEZONE = 127, // PSEUDOTYPE(TIME) -> STRING

    // These access the various components of a time.
    YEAR = 128, // PSEUDOTYPE(TIME) -> NUMBER
    MONTH = 129, // PSEUDOTYPE(TIME) -> NUMBER
    DAY = 130, // PSEUDOTYPE(TIME) -> NUMBER
    DAY_OF_WEEK = 131, // PSEUDOTYPE(TIME) -> NUMBER
    DAY_OF_YEAR = 132, // PSEUDOTYPE(TIME) -> NUMBER
    HOURS = 133, // PSEUDOTYPE(TIME) -> NUMBER
    MINUTES = 134, // PSEUDOTYPE(TIME) -> NUMBER
    SECONDS = 135, // PSEUDOTYPE(TIME) -> NUMBER

    // Construct a time from a date and optional timezone or a
    // date+time and optional timezone.
    TIME = 136, // NUMBER, NUMBER, NUMBER, STRING -> PSEUDOTYPE(TIME) |
            // NUMBER, NUMBER, NUMBER, NUMBER, NUMBER, NUMBER, STRING -> PSEUDOTYPE(TIME) |

    // Constants for ISO 8601 days of the week.
    MONDAY = 107,    // -> 1
    TUESDAY = 108,   // -> 2
    WEDNESDAY = 109, // -> 3
    THURSDAY = 110,  // -> 4
    FRIDAY = 111,    // -> 5
    SATURDAY = 112,  // -> 6
    SUNDAY = 113,    // -> 7

    // Constants for ISO 8601 months.
    JANUARY = 114,   // -> 1
    FEBRUARY = 115,  // -> 2
    MARCH = 116,     // -> 3
    APRIL = 117,     // -> 4
    MAY = 118,       // -> 5
    JUNE = 119,      // -> 6
    JULY = 120,      // -> 7
    AUGUST = 121,    // -> 8
    SEPTEMBER = 122, // -> 9
    OCTOBER = 123,   // -> 10
    NOVEMBER = 124,  // -> 11
    DECEMBER = 125,  // -> 12

    // Indicates to MERGE to replace, or remove in case of an empty literal, the
    // other object rather than merge it.
    LITERAL = 137, // -> Merging
               // JSON -> Merging

    // SEQUENCE, STRING -> GROUPED_SEQUENCE | SEQUENCE, FUNCTION -> GROUPED_SEQUENCE
    GROUP = 144,
    SUM = 145,
    AVG = 146,
    MIN = 147,
    MAX = 148,

    // `str.split()` splits on whitespace
    // `str.split(" ")` splits on spaces only
    // `str.split(" ", 5)` splits on spaces with at most 5 results
    // `str.split(nil, 5)` splits on whitespace with at most 5 results
    SPLIT = 149, // STRING -> ARRAY | STRING, STRING -> ARRAY | STRING, STRING, NUMBER -> ARRAY | STRING, NULL, NUMBER -> ARRAY

    UNGROUP = 150, // GROUPED_DATA -> ARRAY

    // Takes a range of numbers and returns a random number within the range
    RANDOM = 151, // NUMBER, NUMBER {float:BOOL} -> DATUM

    CHANGES = 152, // TABLE -> STREAM
    ARGS = 154, // ARRAY -> SPECIAL (used to splice arguments)

    // BINARY is client-only at the moment, it is not supported on the server
    BINARY = 155, // STRING -> PSEUDOTYPE(BINARY)

    GEOJSON = 157,           // OBJECT -> PSEUDOTYPE(GEOMETRY)
    TO_GEOJSON = 158,        // PSEUDOTYPE(GEOMETRY) -> OBJECT
    POINT = 159,             // NUMBER, NUMBER -> PSEUDOTYPE(GEOMETRY)
    LINE = 160,              // (ARRAY | PSEUDOTYPE(GEOMETRY))... -> PSEUDOTYPE(GEOMETRY)
    POLYGON = 161,           // (ARRAY | PSEUDOTYPE(GEOMETRY))... -> PSEUDOTYPE(GEOMETRY)
    DISTANCE = 162,          // PSEUDOTYPE(GEOMETRY), PSEUDOTYPE(GEOMETRY) {geo_system:STRING, unit:STRING} -> NUMBER
    INTERSECTS = 163,        // PSEUDOTYPE(GEOMETRY), PSEUDOTYPE(GEOMETRY) -> BOOL
    INCLUDES = 164,          // PSEUDOTYPE(GEOMETRY), PSEUDOTYPE(GEOMETRY) -> BOOL
    CIRCLE = 165,            // PSEUDOTYPE(GEOMETRY), NUMBER {num_vertices:NUMBER, geo_system:STRING, unit:STRING, fill:BOOL} -> PSEUDOTYPE(GEOMETRY)
    GET_INTERSECTING = 166,  // TABLE, PSEUDOTYPE(GEOMETRY) {index:!STRING} -> StreamSelection
    FILL = 167,              // PSEUDOTYPE(GEOMETRY) -> PSEUDOTYPE(GEOMETRY)
    GET_NEAREST = 168,       // TABLE, PSEUDOTYPE(GEOMETRY) {index:!STRING, max_results:NUM, max_dist:NUM, geo_system:STRING, unit:STRING} -> ARRAY
    POLYGON_SUB = 171,       // PSEUDOTYPE(GEOMETRY), PSEUDOTYPE(GEOMETRY) -> PSEUDOTYPE(GEOMETRY)

    // Constants for specifying key ranges
    MINVAL = 180,
    MAXVAL = 181,
}
