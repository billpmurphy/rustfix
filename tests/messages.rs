/// Collection of test messages. Invalid messages are prefixed with an 'i'

// Misc test messages

pub const IEMPTY: &'static[u8] = b"";
pub const ITOO_SHORT: &'static[u8] = b"8=FIXT.1.1\x019=0\x0110=00\x01";
pub const IMISSING_FINAL_SOH: &'static[u8] = b"8=FIXT.1.1\x019=0\x0135=G\x0110=000";

// Sample FIX 4.1 messages
// Source: http://fixparser.targetcompid.com/

pub const FIX41_LOGON: &'static[u8] = b"8=FIX.4.1\x019=61\x0135=A\x0134=1\x0149=EXEC\x0152=20121105-23:24:06\x0156=BANZAI\x0198=0\x01108=30\x0110=003\x01";

pub const FIX41_HEARTBEAT: &'static[u8] = b"8=FIX.4.1\x019=49\x0135=0\x0134=2\x0149=BANZAI\x0152=20121105-23:24:37\x0156=EXEC\x0110=228\x01";

pub const FIX41_NEWORDERSINGLE_MKT: &'static[u8] =
b"8=FIX.4.1\x019=103\x0135=D\x0134=3\x0149=BANZAI\x0152=20121105-23:24:42\x0156=EXEC\x0111=1352157882577\x0121=1\x0138=10000\x0140=1\x0154=1\x0155=MSFT\x0159=0\x0110=062\x01";

pub const FIX41_ER_FILL_NEW: &'static[u8] =
b"8=FIX.4.1\x019=139\x0135=8\x0134=3\x0149=EXEC\x0152=20121105-23:24:42\x0156=BANZAI\x016=0\x0111=1352157882577\x0114=0\x0117=1\x0120=0\x0131=0\x0132=0\x0137=1\x0138=10000\x0139=0\x0154=1\x0155=MSFT\x01150=2\x01151=0\x0110=059\x01";

pub const FIX41_ER_FILL_FILLED: &'static[u8] =
b"8=FIX.4.1\x019=153\x0135=8\x0134=4\x0149=EXEC\x0152=20121105-23:24:42\x0156=BANZAI\x016=12.3\x0111=1352157882577\x0114=10000\x0117=2\x0120=0\x0131=12.3\x0132=10000\x0137=2\x0138=10000\x0139=2\x0154=1\x0155=MSFT\x01150=2\x01151=0\x0110=230\x01";

pub const FIX41_NEWORDERSINGLE_LIMIT: &'static[u8] =
b"8=FIX.4.1\x019=108\x0135=D\x0134=5\x0149=BANZAI\x0152=20121105-23:25:12\x0156=EXEC\x0111=1352157912357\x0121=1\x0138=10000\x0140=2\x0144=10\x0154=1\x0155=SPY\x0159=0\x0110=003\x01";

pub const FIX41_CANCEL: &'static[u8] =
b"8=FIX.4.1\x019=104\x0135=F\x0134=6\x0149=BANZAI\x0152=20121105-23:25:16\x0156=EXEC\x0111=1352157916437\x0138=10000\x0141=1352157912357\x0154=1\x0155=SPY\x0110=198\x01";

// Sample FIX 4.1 messages, Invalid
// Source: http://fixparser.targetcompid.com/

pub const IFIX42_INVALIDTYPE: &'static[u8] =
b"8=FIX.4.1\x019=82\x0135=3\x0134=8\x0149=EXEC\x0152=20121105-23:25:16\x0156=BANZAI\x0145=6\x0158=Unsupported message type\x0110=000\x01";
