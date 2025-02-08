use core::ffi::c_void;
use core::ptr::null_mut;
use core::mem::transmute;

use alloc::vec;


use alloc::boxed::Box;
use wdk::{nt_success, println};
use wdk_sys::STATUS_BUFFER_TOO_SMALL;
use wdk_sys::STATUS_INVALID_BUFFER_SIZE;
use wdk_sys::ULONG64;
use wdk_sys::HANDLE;
use wdk_sys::LARGE_INTEGER;
use wdk_sys::UNICODE_STRING;
use wdk_sys::PAGE_SIZE;
use wdk_sys::ntddk::RtlCaptureContext;
use wdk_sys::_CONTEXT;
use wdk_sys::LIST_ENTRY64;
use wdk_sys::CONTEXT_FULL;

use wdk_sys::ntddk::EtwRegister;
use wdk_sys::GUID;
use wdk_sys::NTSTATUS;
use wdk_sys::{ULONG, PVOID};
use wdk_sys::ntddk::KeQueryPerformanceCounter;

use wdk_sys::ntddk::MmGetSystemRoutineAddress;
use crate::utils::ToUnicodeString;

const  SystemTraceControlGuid : GUID = GUID {
    Data1: 0x9e814aad,
    Data2: 0x3204,
    Data3: 0x11d2,
    Data4: [0x9a, 0x82, 0x00, 0x60, 0x08, 0xa8, 0x69, 0x39],
};
const SystemPerformanceTraceInformation : ULONG =  0x1F;
/////////////////////// added from chinese github /////////////////////

/*
#define EtwpStartTrace		1
#define EtwpStopTrace		2
#define EtwpQueryTrace		3
#define EtwpUpdateTrace		4
#define EtwpFlushTrace		5

#define WNODE_FLAG_TRACED_GUID			0x00020000  // denotes a trace
#define EVENT_TRACE_BUFFERING_MODE      0x00000400  // Buffering mode only
#define EVENT_TRACE_FLAG_SYSTEMCALL     0x00000080  // system calls
 */

const EtwpStartTrace: ULONG = 1;
const EtwpStopTrace: ULONG = 2;
const EtwpQueryTrace: ULONG = 3;
const EtwpUpdateTrace: ULONG = 4;
const EtwpFlushTrace: ULONG = 5;

const WNODE_FLAG_TRACED_GUID: ULONG = 0x00020000;
const EVENT_TRACE_BUFFERING_MODE: ULONG = 0x00000400;
const EVENT_TRACE_FLAG_SYSTEMCALL: ULONG = 0x00000080;



/*

{
	ULONG BufferSize;        // Size of entire buffer inclusive of this ULONG
	ULONG ProviderId;    // Provider Id of driver returning this buffer
	union
	{
		ULONG64 HistoricalContext;  // Logger use
		struct
		{
			ULONG Version;           // Reserved
			ULONG Linkage;           // Linkage field reserved for WMI
		} DUMMYSTRUCTNAME;
	} DUMMYUNIONNAME;

	union
	{
		ULONG CountLost;         // Reserved
		HANDLE KernelHandle;     // Kernel handle for data block
		LARGE_INTEGER TimeStamp; // Timestamp as returned in units of 100ns
								 // since 1/1/1601
	} DUMMYUNIONNAME2;
	GUID Guid;                  // Guid for data block returned with results
	ULONG ClientContext;
	ULONG Flags;             // Flags, see below
}
*/

#[repr(C)]
struct VersionLinkage {
    Version: ULONG,
    Linkage: ULONG,
}


impl Copy for VersionLinkage {}

impl Clone for VersionLinkage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
union wnode_reserved_header_union1 {
    HistoricalContext: ULONG64,
    versionLinkage: VersionLinkage,
}

#[repr(C)]
union wnode_reserved_header_union2 {
    CountLost: ULONG,
    KernelHandle: HANDLE,
    TimeStamp: LARGE_INTEGER,
    
}

#[repr(C)]
struct wnode_header {
    BufferSize: ULONG,
    ProviderId: ULONG,
    reserved1: wnode_reserved_header_union1,
    reserved2: wnode_reserved_header_union2,
    Guid: GUID,
    ClientContext: ULONG,
    Flags: ULONG,
}


/*
typedef struct _EVENT_TRACE_PROPERTIES {
	WNODE_HEADER	Wnode;
	ULONG			BufferSize;
	ULONG			MinimumBuffers;
	ULONG			MaximumBuffers;
	ULONG			MaximumFileSize;
	ULONG			LogFileMode;
	ULONG			FlushTimer;
	ULONG			EnableFlags;
	LONG			AgeLimit;
	ULONG			NumberOfBuffers;
	ULONG			FreeBuffers;
	ULONG			EventsLost;
	ULONG			BuffersWritten;
	ULONG			LogBuffersLost;
	ULONG			RealTimeBuffersLost;
	HANDLE			LoggerThreadId;
	ULONG			LogFileNameOffset;
	ULONG			LoggerNameOffset;
} EVENT_TRACE_PROPERTIES, * PEVENT_TRACE_PROPERTIES;
*/

#[repr(C)]
struct event_trace_properties {
    Wnode: wnode_header,
    BufferSize: ULONG,
    MinimumBuffers: ULONG,
    MaximumBuffers: ULONG,
    MaximumFileSize: ULONG,
    LogFileMode: ULONG,
    FlushTimer: ULONG,
    EnableFlags: ULONG,
    AgeLimit: i32,
    NumberOfBuffers: ULONG,
    FreeBuffers: ULONG,
    EventsLost: ULONG,
    BuffersWritten: ULONG,
    LogBuffersLost: ULONG,
    RealTimeBuffersLost: ULONG,
    LoggerThreadId: HANDLE,
    LogFileNameOffset: ULONG,
    LoggerNameOffset: ULONG,
    Unknown: [ULONG64; 3],
    ProviderName: UNICODE_STRING,
}


/*
typedef struct _CKCL_TRACE_PROPERIES : EVENT_TRACE_PROPERTIES
{
	ULONG64					Unknown[3];
	UNICODE_STRING			ProviderName;
} CKCL_TRACE_PROPERTIES, * PCKCL_TRACE_PROPERTIES;
*/


#[repr(C)]
struct ckcl_trace_properties {
    event_trace_properties: event_trace_properties,
    Unknown: [ULONG64; 3],
    ProviderName: UNICODE_STRING,
}



const CkclSessionGuid : GUID = GUID {
    Data1: 0x54dea73a,
    Data2: 0xed1f,
    Data3: 0x42a4,
    Data4: [0xaf, 0x71, 0x3e, 0x63, 0xd0, 0x56, 0xf1, 0x74],
};



/*
typedef enum _EVENT_TRACE_INFORMATION_CLASS {
	EventTraceKernelVersionInformation,
	EventTraceGroupMaskInformation,
	EventTracePerformanceInformation,
	EventTraceTimeProfileInformation,
	EventTraceSessionSecurityInformation,
	EventTraceSpinlockInformation,
	EventTraceStackTracingInformation,
	EventTraceExecutiveResourceInformation,
	EventTraceHeapTracingInformation,
	EventTraceHeapSummaryTracingInformation,
	EventTracePoolTagFilterInformation,
	EventTracePebsTracingInformation,
	EventTraceProfileConfigInformation,
	EventTraceProfileSourceListInformation,
	EventTraceProfileEventListInformation,
	EventTraceProfileCounterListInformation,
	EventTraceStackCachingInformation,
	EventTraceObjectTypeFilterInformation,
	MaxEventTraceInfoClass
} EVENT_TRACE_INFORMATION_CLASS;

*/

#[repr(C)]
enum EVENT_TRACE_INFORMATION_CLASS {
    EventTraceKernelVersionInformation,
    EventTraceGroupMaskInformation,
    EventTracePerformanceInformation,
    EventTraceTimeProfileInformation,
    EventTraceSessionSecurityInformation,
    EventTraceSpinlockInformation,
    EventTraceStackTracingInformation,
    EventTraceExecutiveResourceInformation,
    EventTraceHeapTracingInformation,
    EventTraceHeapSummaryTracingInformation,
    EventTracePoolTagFilterInformation,
    EventTracePebsTracingInformation,
    EventTraceProfileConfigInformation,
    EventTraceProfileSourceListInformation,
    EventTraceProfileEventListInformation,
    EventTraceProfileCounterListInformation,
    EventTraceStackCachingInformation,
    EventTraceObjectTypeFilterInformation,
    MaxEventTraceInfoClass,
}


/*
typedef struct _EVENT_TRACE_SYSTEM_EVENT_INFORMATION
{
	EVENT_TRACE_INFORMATION_CLASS EventTraceInformationClass;
	HANDLE TraceHandle;
	ULONG HookId[1];
} EVENT_TRACE_SYSTEM_EVENT_INFORMATION, * PEVENT_TRACE_SYSTEM_EVENT_INFORMATION;
*/

#[repr(C)]
struct EVENT_TRACE_SYSTEM_EVENT_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    TraceHandle: HANDLE,
    HookId: ULONG,
}


/*

typedef struct _EVENT_TRACE_PROFILE_COUNTER_INFORMATION
{
	EVENT_TRACE_INFORMATION_CLASS EventTraceInformationClass;
	HANDLE TraceHandle;
	ULONG ProfileSource[1];
} EVENT_TRACE_PROFILE_COUNTER_INFORMATION, * PEVENT_TRACE_PROFILE_COUNTER_INFORMATION;
*/

#[repr(C)]
struct EVENT_TRACE_PROFILE_COUNTER_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    TraceHandle: HANDLE,
    ProfileSource: ULONG,
}


/*
NTSTATUS EtwInitilizer::open_pmc_counter()
{
	auto status = STATUS_SUCCESS;
	auto pmc_count_info = (PEVENT_TRACE_PROFILE_COUNTER_INFORMATION)(nullptr);
	auto pmc_event_info=(PEVENT_TRACE_SYSTEM_EVENT_INFORMATION)(nullptr);
	constexpr auto syscall_hookid = 0xf33ul;



	if (!__is_open) return STATUS_FLT_NOT_INITIALIZED;

	do {

		/*获取ckcl_context的loggerid*/
		auto EtwpDebuggerData=reinterpret_cast<ULONG***>( \
			kstd::SysInfoManager::getInstance()->getSysInfo()->EtwpDebuggerData);
		
		if (!EtwpDebuggerData) {
			status = STATUS_NOT_SUPPORTED;
			LOG_ERROR("failed to get EtwpDebuggerData!\r\n");
		}
		
		/*这个可以参考第一版的ETW HOOK，这里简写了*/
		auto logger_id = EtwpDebuggerData[2][2][0];

		pmc_count_info = kalloc<EVENT_TRACE_PROFILE_COUNTER_INFORMATION>(NonPagedPool);
		if (!pmc_count_info) {
			LOG_ERROR("failed to alloc memory for pmc_count!\r\n");
			status = STATUS_MEMORY_NOT_ALLOCATED;
			break;
		}
		//先设置PMC Count 我们只关心一个hookid 那就是syscall的hookid 0xf33 profile source 随便设置
		pmc_count_info->EventTraceInformationClass = EventTraceProfileCounterListInformation;
		pmc_count_info->TraceHandle = ULongToHandle(logger_id)/*这个其实就是loggerid*/;
		pmc_count_info->ProfileSource[0] = 1;/*随便填写*/

		auto EtwpMaxPmcCounter=get_EtwpMaxPmcCounter();

		auto org = (unsigned char)0;

		if (MmIsAddressValid(EtwpMaxPmcCounter)) {

			org = *EtwpMaxPmcCounter;

			if (org <= 1) *EtwpMaxPmcCounter = 2;

		}

		status=ZwSetSystemInformation(SystemPerformanceTraceInformation, pmc_count_info, sizeof EVENT_TRACE_PROFILE_COUNTER_INFORMATION);
		if (!NT_SUCCESS(status)) {
			LOG_ERROR("failed to configure pmc counter,errcode=%x\r\n", status);
			break;
		}


		if (MmIsAddressValid(EtwpMaxPmcCounter)) {
			if (org <= 1) *EtwpMaxPmcCounter = org;
		}

		//然后设置PMC Event hookid只需要一个就行
		pmc_event_info = kalloc<EVENT_TRACE_SYSTEM_EVENT_INFORMATION>(NonPagedPool);
		if (!pmc_event_info) {
			LOG_ERROR("failed to alloc memory for pmc_event_info!\r\n");
			status = STATUS_MEMORY_NOT_ALLOCATED;
			break;
		}

		pmc_event_info->EventTraceInformationClass = EventTraceProfileEventListInformation;
		pmc_event_info->TraceHandle = ULongToHandle(logger_id);
		pmc_event_info->HookId[0] = syscall_hookid;/*必须0xf33*/


		status = ZwSetSystemInformation(SystemPerformanceTraceInformation, pmc_event_info, sizeof EVENT_TRACE_SYSTEM_EVENT_INFORMATION);
		if (!NT_SUCCESS(status)) {
			LOG_ERROR("failed to configure pmc event,errcode=%x\r\n", status);
			break;
		}

		

	} while (false);

	//clean up
	if (pmc_count_info) ExFreePool(pmc_count_info);
	if (pmc_event_info) ExFreePool(pmc_event_info);

	return status;
} */


#[repr(C)]
pub struct DBGKD_DEBUG_DATA_HEADER64 {
    pub List: LIST_ENTRY64,
    pub OwnerTag: u32,
    pub Size: u32,
}

#[repr(C)]
pub struct KDDEBUGGER_DATA64 {

    pub Header: DBGKD_DEBUG_DATA_HEADER64,
    pub KernBase: u64,
    pub BreakpointWithStatus: u64,
    pub SavedContext: u64,
    pub ThCallbackStack: u16,
    pub NextCallback: u16,
    pub FramePointer: u16,
    pub _bitfield: u16,
    pub KiCallUserMode: u64,
    pub KeUserCallbackDispatcher: u64,
    pub PsLoadedModuleList: u64,
    pub PsActiveProcessHead: u64,
    pub PspCidTable: u64,
    pub ExpSystemResourcesList: u64,
    pub ExpPagedPoolDescriptor: u64,
    pub ExpNumberOfPagedPools: u64,
    pub KeTimeIncrement: u64,
    pub KeBugCheckCallbackListHead: u64,
    pub KiBugcheckData: u64,
    pub IopErrorLogListHead: u64,
    pub ObpRootDirectoryObject: u64,
    pub ObpTypeObjectType: u64,
    pub MmSystemCacheStart: u64,
    pub MmSystemCacheEnd: u64,
    pub MmSystemCacheWs: u64,
    pub MmPfnDatabase: u64,
    pub MmSystemPtesStart: u64,
    pub MmSystemPtesEnd: u64,
    pub MmSubsectionBase: u64,
    pub MmNumberOfPagingFiles: u64,
    pub MmLowestPhysicalPage: u64,
    pub MmHighestPhysicalPage: u64,
    pub MmNumberOfPhysicalPages: u64,
    pub MmMaximumNonPagedPoolInBytes: u64,
    pub MmNonPagedSystemStart: u64,
    pub MmNonPagedPoolStart: u64,
    pub MmNonPagedPoolEnd: u64,
    pub MmPagedPoolStart: u64,
    pub MmPagedPoolEnd: u64,
    pub MmPagedPoolInformation: u64,
    pub MmPageSize: u64,
    pub MmSizeOfPagedPoolInBytes: u64,
    pub MmTotalCommitLimit: u64,
    pub MmTotalCommittedPages: u64,
    pub MmSharedCommit: u64,
    pub MmDriverCommit: u64,
    pub MmProcessCommit: u64,
    pub MmPagedPoolCommit: u64,
    pub MmExtendedCommit: u64,
    pub MmZeroedPageListHead: u64,
    pub MmFreePageListHead: u64,
    pub MmStandbyPageListHead: u64,
    pub MmModifiedPageListHead: u64,
    pub MmModifiedNoWritePageListHead: u64,
    pub MmAvailablePages: u64,
    pub MmResidentAvailablePages: u64,
    pub PoolTrackTable: u64,
    pub NonPagedPoolDescriptor: u64,
    pub MmHighestUserAddress: u64,
    pub MmSystemRangeStart: u64,
    pub MmUserProbeAddress: u64,
    pub KdPrintCircularBuffer: u64,
    pub KdPrintCircularBufferEnd: u64,
    pub KdPrintWritePointer: u64,
    pub KdPrintRolloverCount: u64,
    pub MmLoadedUserImageList: u64,
    pub NtBuildLab: u64,
    pub KiNormalSystemCall: u64,
    pub KiProcessorBlock: u64,
    pub MmUnloadedDrivers: u64,
    pub MmLastUnloadedDriver: u64,
    pub MmTriageActionTaken: u64,
    pub MmSpecialPoolTag: u64,
    pub KernelVerifier: u64,
    pub MmVerifierData: u64,
    pub MmAllocatedNonPagedPool: u64,
    pub MmPeakCommitment: u64,
    pub MmTotalCommitLimitMaximum: u64,
    pub CmNtCSDVersion: u64,
    pub MmPhysicalMemoryBlock: u64,
    pub MmSessionBase: u64,
    pub MmSessionSize: u64,
    pub MmSystemParentTablePage: u64,
    pub MmVirtualTranslationBase: u64,
    pub OffsetKThreadNextProcessor: u16,
    pub OffsetKThreadTeb: u16,
    pub OffsetKThreadKernelStack: u16,
    pub OffsetKThreadInitialStack: u16,
    pub OffsetKThreadApcProcess: u16,
    pub OffsetKThreadState: u16,
    pub OffsetKThreadBStore: u16,
    pub OffsetKThreadBStoreLimit: u16,
    pub SizeEProcess: u16,
    pub OffsetEprocessPeb: u16,
    pub OffsetEprocessParentCID: u16,
    pub OffsetEprocessDirectoryTableBase: u16,
    pub SizePrcb: u16,
    pub OffsetPrcbDpcRoutine: u16,
    pub OffsetPrcbCurrentThread: u16,
    pub OffsetPrcbMhz: u16,
    pub OffsetPrcbCpuType: u16,
    pub OffsetPrcbVendorString: u16,
    pub OffsetPrcbProcStateContext: u16,
    pub OffsetPrcbNumber: u16,
    pub SizeEThread: u16,
    pub L1tfHighPhysicalBitIndex: u8,
    pub L1tfSwizzleBitIndex: u8,
    pub Padding0: u32,
    pub KdPrintCircularBufferPtr: u64,
    pub KdPrintBufferSize: u64,
    pub KeLoaderBlock: u64,
    pub SizePcr: u16,
    pub OffsetPcrSelfPcr: u16,
    pub OffsetPcrCurrentPrcb: u16,
    pub OffsetPcrContainedPrcb: u16,
    pub OffsetPcrInitialBStore: u16,
    pub OffsetPcrBStoreLimit: u16,
    pub OffsetPcrInitialStack: u16,
    pub OffsetPcrStackLimit: u16,
    pub OffsetPrcbPcrPage: u16,
    pub OffsetPrcbProcStateSpecialReg: u16,
    pub GdtR0Code: u16,
    pub GdtR0Data: u16,
    pub GdtR0Pcr: u16,
    pub GdtR3Code: u16,
    pub GdtR3Data: u16,
    pub GdtR3Teb: u16,
    pub GdtLdt: u16,
    pub GdtTss: u16,
    pub Gdt64R3CmCode: u16,
    pub Gdt64R3CmTeb: u16,
    pub IopNumTriageDumpDataBlocks: u64,
    pub IopTriageDumpDataBlocks: u64,
    pub VfCrashDataBlock: u64,
    pub MmBadPagesDetected: u64,
    pub MmZeroedPageSingleBitErrorsDetected: u64,
    pub EtwpDebuggerData: u64,
    pub OffsetPrcbContext: u16,
    pub OffsetPrcbMaxBreakpoints: u16,
    pub OffsetPrcbMaxWatchpoints: u16,
    pub OffsetKThreadStackLimit: u32,
    pub OffsetKThreadStackBase: u32,
    pub OffsetKThreadQueueListEntry: u32,
    pub OffsetEThreadIrpList: u32,
    pub OffsetPrcbIdleThread: u16,
    pub OffsetPrcbNormalDpcState: u16,
    pub OffsetPrcbDpcStack: u16,
    pub OffsetPrcbIsrStack: u16,
    pub SizeKDPC_STACK_FRAME: u16,
    pub OffsetKPriQueueThreadListHead: u16,
    pub OffsetKThreadWaitReason: u16,
    pub Padding1: u16,
    pub PteBase: u64,
    pub RetpolineStubFunctionTable: u64,
    pub RetpolineStubFunctionTableSize: u32,
    pub RetpolineStubOffset: u32,
    pub RetpolineStubSize: u32,
    pub OffsetEProcessMmHotPatchContext: u16,
    pub OffsetKThreadShadowStackLimit: u32,
    pub OffsetKThreadShadowStackBase: u32,
    pub ShadowStackEnabled: u64,
    pub PointerAuthMask: u64,
    pub OffsetPrcbExceptionStack: u16,
}
const KDDEBUGGER_DATA_OFFSET: usize = 0x2080;

type KeCapturePersistentThreadState = unsafe extern "C" fn(
    context: *mut _CONTEXT,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
    arg6: u32,
    buffer: *mut c_void,
);

pub fn get_ckcl_logger_id() -> ULONG {
    let mut logger_id = 0;
    unsafe {
        let mut context = Box::new(_CONTEXT::default());
        context.ContextFlags = CONTEXT_FULL;
        RtlCaptureContext(&mut *context);
        


        let mut func_name = "KeCapturePersistentThreadState".to_unicode();
        let addr = MmGetSystemRoutineAddress(&mut func_name);

        let func = transmute::<*mut c_void, KeCapturePersistentThreadState>(addr);

        let mut tmp_buffer = vec![0u8; 0x40000];
        let buffer = tmp_buffer.as_mut_ptr() as *mut c_void;


        func(&mut *context, 0, 0, 0, 0, 0, 0, buffer);

        let mut dump_header: KDDEBUGGER_DATA64 = unsafe { core::mem::zeroed() };
        let src = (buffer as *const u8).add(KDDEBUGGER_DATA_OFFSET) as *const KDDEBUGGER_DATA64;
        core::ptr::copy_nonoverlapping(src, &mut dump_header, 1);


        

        let etwp_debugger_data_ptr = dump_header.EtwpDebuggerData as *const *const *const u32;

        let logger_id = unsafe { *(*etwp_debugger_data_ptr.add(2)).add(2).add(0) };
        println!("logger_id: {:?}", *logger_id);
        let int_logger_id = *logger_id as ULONG;
        return int_logger_id;
    }
    
}


pub fn pmc_counter_enable() {
    let logger_id = get_ckcl_logger_id();
    let mut pmc_count_info = Box::new(EVENT_TRACE_PROFILE_COUNTER_INFORMATION {
        EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS::EventTraceProfileCounterListInformation,
        TraceHandle: logger_id as HANDLE,
        ProfileSource: 0x524,
    });

    let status = unsafe {
        ZwSetSystemInformation(
            SystemPerformanceTraceInformation,
            &mut *pmc_count_info as *mut EVENT_TRACE_PROFILE_COUNTER_INFORMATION as PVOID,
            0x14 as ULONG,
        )
    };


    println!("ZwSetSystemInformation: {:?}", status);


    let mut pmc_event_info = Box::new(EVENT_TRACE_SYSTEM_EVENT_INFORMATION {
        EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS::EventTraceProfileEventListInformation,
        TraceHandle: logger_id as HANDLE,
        HookId: 0xf33,
    });

    let status = unsafe {
        ZwSetSystemInformation(
            SystemPerformanceTraceInformation,
            &mut *pmc_event_info as *mut EVENT_TRACE_SYSTEM_EVENT_INFORMATION as PVOID,
            0x14 as ULONG,
        )
    };
    println!("ZwSetSystemInformation: {:?}", status);

    
    
}



pub fn etw_init() {


    let provider_name = "Circular Kernel Context Logger".to_unicode();

    let mut zwcontrol_ckcl_start = event_trace_properties{
        Wnode: wnode_header {
            BufferSize: size_of::<ckcl_trace_properties>() as ULONG,
            ProviderId: 0x0,
            reserved1: wnode_reserved_header_union1 {
                HistoricalContext: 0,
            },
            reserved2: wnode_reserved_header_union2 {
                CountLost: 0,
            },
            Guid: CkclSessionGuid,
            ClientContext: 1,
            Flags: WNODE_FLAG_TRACED_GUID,
        },
        BufferSize: PAGE_SIZE * 16,
        MinimumBuffers: 0x2,
        MaximumBuffers: 0x2,
        MaximumFileSize: 0x0,
        LogFileMode: EVENT_TRACE_BUFFERING_MODE,
        FlushTimer: 0x3,
        EnableFlags: 0x0,
        AgeLimit: 0x0,
        NumberOfBuffers: 0x0,
        FreeBuffers: 0x0,
        EventsLost: 0x0,
        BuffersWritten: 0x0,
        LogBuffersLost: 0x0,
        RealTimeBuffersLost: 0x0,
        LoggerThreadId: null_mut(),
        LogFileNameOffset: 0x0,
        LoggerNameOffset: 0x0,
        Unknown: [0; 3],
        ProviderName: provider_name,
    };

    let returnlength: *mut ULONG = Box::into_raw(Box::new(0));
    
    zwcontrol_ckcl_start.Wnode.BufferSize = size_of::<ckcl_trace_properties>() as ULONG;
    unsafe {
    let ckcl_ptr: *mut c_void = &mut zwcontrol_ckcl_start as *mut _ as *mut c_void;

        println!("ckcl_ptr: {:?}", ckcl_ptr);
        let status = ZwTraceControl(EtwpStartTrace, ckcl_ptr, size_of::<ckcl_trace_properties>() as ULONG, ckcl_ptr, size_of::<ckcl_trace_properties>() as ULONG, returnlength);
        println!("ZwTraceControl: {:?}", status);


        println!("ckcl_ptr: {:?}", ckcl_ptr);
        zwcontrol_ckcl_start.EnableFlags = EVENT_TRACE_FLAG_SYSTEMCALL;
        let status = ZwTraceControl(EtwpUpdateTrace, ckcl_ptr, size_of::<ckcl_trace_properties>() as ULONG, ckcl_ptr, size_of::<ckcl_trace_properties>() as ULONG, returnlength);
        println!("ZwTraceControl: {:?}", status);
    }
    

}



///////////////////////////// end of added from chinese github /////////////////////////


pub fn hooking_prototype() {

    

    unsafe {
        let FunctionCode = 0x1;
        let InBuffer: *mut c_void = [0;0xbB0].as_mut_ptr() as *mut c_void; 
        let InBufferLen = 0xbB0;
        let OutBuffer: *mut c_void = [0;0xbB0].as_mut_ptr() as *mut c_void;
        let OutBufferLen = 0xbB0;
        let ReturnLength: [u32; 1] = [0];

        let status = ZwTraceControl(FunctionCode, InBuffer , InBufferLen, OutBuffer, OutBufferLen, ReturnLength.as_ptr() as *mut ULONG);
        println!("ZwTraceControl: {:?}", status);
        println!("ReturnLength: {:?}", ReturnLength);
        if status == STATUS_BUFFER_TOO_SMALL {
            println!("STATUS_BUFFER_TOO_SMALL");
        }
        else if status ==  STATUS_INVALID_BUFFER_SIZE {
            println!("STATUS_INVALID_BUFFER_SIZE");
            
        }
        else {
            println!("STATUS: is not STATUS_BUFFER_TOO_SMALL");
        }
    }

    



}


pub fn hooking() {
    let mut name = "HalPrivateDispatchTable".to_unicode();
    unsafe {
        let address = MmGetSystemRoutineAddress( &mut name);
        println!("HalPrivateDispatchTable: {:?}", address);
        let HalpCollectPmcCounters = address.wrapping_add(0x248);
        println!("HalpCollectPmcCounters: {:?}", HalpCollectPmcCounters);

        let func_ptr: extern "C" fn() -> () = base_hook;


        let void_ptr = func_ptr as *mut c_void;
        *(HalpCollectPmcCounters as *mut *mut c_void) = void_ptr;
    }
    
}
extern "C"  fn base_hook() {
    println!("base_hook");
}


extern "system" {
    pub fn ZwTraceControl(
        FunctionCode: ULONG,
        InBuffer: PVOID,
        InBufferLen: ULONG,
        OutBuffer: PVOID,
        OutBufferLen: ULONG,
        ReturnLength: *mut ULONG,
    ) -> NTSTATUS;
}

extern "system" {
    pub fn ZwSetSystemInformation(
        SystemInformationClass: ULONG,
        SystemInformation: PVOID,
        SystemInformationLength: ULONG,
    ) -> NTSTATUS;
}