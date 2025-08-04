use libc::{c_char, c_int, c_void};
use super::NgSpiceManager;

pub unsafe extern fn cbw_send_char<T>(msg: *const c_char, id: c_int, user: *const c_void) -> c_int where T: NgSpiceManager{
    unsafe {
        <T as NgSpiceManager>::send_char(&mut *(user as *mut T), std::ffi::CStr::from_ptr(msg).to_str().unwrap().to_owned(), id);
    }
    0
}

// /* callback functions
// addresses received from caller with ngSpice_Init() function
// */
// /* sending output from stdout, stderr to caller */
// typedef int (SendChar)(char*, int, void*);
// /*
//    char* string to be sent to caller output
//    int   identification number of calling ngspice shared lib
//    void* return pointer received from caller, e.g. pointer to object having sent the request
// */
// /* sending simulation status to caller */
// typedef int (SendStat)(char*, int, void*);
// /*
//    char* simulation status and value (in percent) to be sent to caller
//    int   identification number of calling ngspice shared lib
//    void* return pointer received from caller
// */
// /* asking for controlled exit */
// typedef int (ControlledExit)(int, NG_BOOL, NG_BOOL, int, void*);
// /*
//    int   exit status
//    NG_BOOL  if true: immediate unloading dll, if false: just set flag, unload is done when function has returned
//    NG_BOOL  if true: exit upon 'quit', if false: exit due to ngspice.dll error
//    int   identification number of calling ngspice shared lib
//    void* return pointer received from caller
// */
// /* send back actual vector data */
// typedef int (SendData)(pvecvaluesall, int, int, void*);
// /*
//    vecvaluesall* pointer to array of structs containing actual values from all vectors
//    int           number of structs (one per vector)
//    int           identification number of calling ngspice shared lib
//    void*         return pointer received from caller
// */

// /* send back initailization vector data */
// typedef int (SendInitData)(pvecinfoall, int, void*);
// /*
//    vecinfoall* pointer to array of structs containing data from all vectors right after initialization
//    int         identification number of calling ngspice shared lib
//    void*       return pointer received from caller
// */

// /* indicate if background thread is running */
// typedef int (BGThreadRunning)(NG_BOOL, int, void*);
// /*
//    NG_BOOL        true if background thread is running
//    int         identification number of calling ngspice shared lib
//    void*       return pointer received from caller
// */

// /* callback functions
//    addresses received from caller with ngSpice_Init_Sync() function
// */

// /* ask for VSRC EXTERNAL value */
// typedef int (GetVSRCData)(double*, double, char*, int, void*);
// /*
//    double*     return voltage value
//    double      actual time
//    char*       node name
//    int         identification number of calling ngspice shared lib
//    void*       return pointer received from caller
// */

// /* ask for ISRC EXTERNAL value */
// typedef int (GetISRCData)(double*, double, char*, int, void*);
// /*
//    double*     return current value
//    double      actual time
//    char*       node name
//    int         identification number of calling ngspice shared lib
//    void*       return pointer received from caller
// */

// /* ask for new delta time depending on synchronization requirements */
// typedef int (GetSyncData)(double, double*, double, int, int, int, void*);
// /*
//    double      actual time (ckt->CKTtime)
//    double*     delta time (ckt->CKTdelta)
//    double      old delta time (olddelta)
//    int         redostep (as set by ngspice)
//    int         identification number of calling ngspice shared lib
//    int         location of call for synchronization in dctran.c
//    void*       return pointer received from caller
// */

// #ifdef XSPICE
// /* callback functions
// addresses received from caller with ngSpice_Init_Evt() function
// */

// /* Upon time step finished, called per node */
// typedef int (SendEvtData)(int, double, double, char *, void *, int, int, int, void*);
// /*
//    int         node index
//    double      step, actual simulation time
//    double      dvalue, a real value for specified structure component for plotting purposes
//    char        *svalue, a string value for specified structure component for printing
//    void        *pvalue, a binary data structure
//    int         plen, size of the *pvalue structure
//    int         mode, the mode (op, dc, tran) we are in
//    int         ident, identification number of calling ngspice shared lib
//    void*       return pointer received from caller
// */

// /* Upon initialization, called once per event node
//    To build up a dictionary of nodes */
// typedef int (SendInitEvtData)(int, int, char*, char*, int, void*);
// /*
//    int         node index
//    int         maximum node index, number of nodes
//    char*       node name
//    char*       udn-name, node type
//    int         identification number of calling ngspice shared lib
//    void*       return pointer received from caller
// */
// #endif