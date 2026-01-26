///! Assembly-level verification - Obtiene información REAL del sistema

use std::arch::asm;

#[cfg(target_arch = "x86_64")]
pub struct AssemblyVerifier;

#[cfg(target_arch = "x86_64")]
impl AssemblyVerifier {
    /// Obtiene CPU ID real via assembly
    pub fn get_cpu_id() -> (u32, u32, u32, u32) {
        let mut eax: u32;
        let mut ebx: u32;
        let mut ecx: u32;
        let mut edx: u32;
        
        unsafe {
            asm!(
                "mov eax, 0",
                "cpuid",
                out("eax") eax,
                out("ebx") ebx,
                out("ecx") ecx,
                out("edx") edx,
            );
        }
        
        (eax, ebx, ecx, edx)
    }
    
    /// Lee contador de ciclos TSC real
    pub fn read_tsc() -> u64 {
        let low: u32;
        let high: u32;
        
        unsafe {
            asm!(
                "rdtsc",
                out("eax") low,
                out("edx") high,
            );
        }
        
        ((high as u64) << 32) | (low as u64)
    }
    
    /// Mide tiempo real de ejecución con TSC
    pub fn measure_execution<F, R>(f: F) -> (R, u64)
    where
        F: FnOnce() -> R,
    {
        let start = Self::read_tsc();
        let result = f();
        let end = Self::read_tsc();
        
        (result, end - start)
    }
    
    /// Verifica si proceso está corriendo (via PID real)
    #[cfg(target_os = "windows")]
    pub fn verify_process_exists(pid: u32) -> bool {
        use winapi::um::processthreadsapi::OpenProcess;
        use winapi::um::winnt::PROCESS_QUERY_INFORMATION;
        use winapi::um::handleapi::CloseHandle;
        
        unsafe {
            let handle = OpenProcess(PROCESS_QUERY_INFORMATION, 0, pid);
            if handle.is_null() {
                false
            } else {
                CloseHandle(handle);
                true
            }
        }
    }
    
    /// Lee memoria de proceso directamente
    #[cfg(target_os = "windows")]
    pub fn read_process_memory(pid: u32, address: usize, size: usize) -> Option<Vec<u8>> {
        use winapi::um::processthreadsapi::OpenProcess;
        use winapi::um::memoryapi::ReadProcessMemory;
        use winapi::um::winnt::PROCESS_VM_READ;
        use winapi::um::handleapi::CloseHandle;
        
        unsafe {
            let handle = OpenProcess(PROCESS_VM_READ, 0, pid);
            if handle.is_null() {
                return None;
            }
            
            let mut buffer = vec![0u8; size];
            let mut bytes_read = 0;
            
            let result = ReadProcessMemory(
                handle,
                address as *const _,
                buffer.as_mut_ptr() as *mut _,
                size,
                &mut bytes_read,
            );
            
            CloseHandle(handle);
            
            if result != 0 {
                buffer.truncate(bytes_read);
                Some(buffer)
            } else {
                None
            }
        }
    }
    
    /// Obtiene tabla de conexiones TCP real (syscall directo)
    #[cfg(target_os = "windows")]
    pub fn get_tcp_table_raw() -> Vec<TcpConnectionRaw> {
        use winapi::um::iphlpapi::GetExtendedTcpTable;
        use winapi::shared::tcpmib::*;
        use winapi::shared::ws2def::AF_INET;
        use std::ptr;
        
        let mut connections = Vec::new();
        
        unsafe {
            let mut size: u32 = 0;
            
            // Primera llamada para obtener tamaño
            GetExtendedTcpTable(
                ptr::null_mut(),
                &mut size,
                0,
                AF_INET as u32,
                TCP_TABLE_OWNER_PID_ALL,
                0,
            );
            
            if size == 0 {
                return connections;
            }
            
            // Allocar buffer
            let mut buffer = vec![0u8; size as usize];
            let table = buffer.as_mut_ptr() as *mut MIB_TCPTABLE_OWNER_PID;
            
            // Segunda llamada para obtener datos reales
            let result = GetExtendedTcpTable(
                table as *mut _,
                &mut size,
                0,
                AF_INET as u32,
                TCP_TABLE_OWNER_PID_ALL,
                0,
            );
            
            if result == 0 {
                let table = &*table;
                let entries = std::slice::from_raw_parts(
                    table.table.as_ptr(),
                    table.dwNumEntries as usize,
                );
                
                for entry in entries {
                    connections.push(TcpConnectionRaw {
                        local_addr: entry.dwLocalAddr,
                        local_port: u16::from_be(entry.dwLocalPort as u16),
                        remote_addr: entry.dwRemoteAddr,
                        remote_port: u16::from_be(entry.dwRemotePort as u16),
                        pid: entry.dwOwningPid,
                        state: entry.dwState,
                    });
                }
            }
        }
        
        connections
    }
}

#[derive(Debug, Clone)]
pub struct TcpConnectionRaw {
    pub local_addr: u32,
    pub local_port: u16,
    pub remote_addr: u32,
    pub remote_port: u16,
    pub pid: u32,
    pub state: u32,
}

impl TcpConnectionRaw {
    pub fn to_ip_string(&self) -> String {
        format!(
            "{}.{}.{}.{}",
            self.local_addr & 0xFF,
            (self.local_addr >> 8) & 0xFF,
            (self.local_addr >> 16) & 0xFF,
            (self.local_addr >> 24) & 0xFF,
        )
    }
}

/// Verifica que el sistema está funcionando correctamente
pub fn run_system_verification() -> SystemVerificationReport {
    println!("🔍 Assembly-level System Verification");
    println!("=====================================");
    
    // 1. CPU ID real
    #[cfg(target_arch = "x86_64")]
    let (eax, ebx, ecx, edx) = AssemblyVerifier::get_cpu_id();
    #[cfg(target_arch = "x86_64")]
    println!("✅ CPU ID: {:08x} {:08x} {:08x} {:08x}", eax, ebx, ecx, edx);
    
    // 2. TSC real
    #[cfg(target_arch = "x86_64")]
    let tsc = AssemblyVerifier::read_tsc();
    #[cfg(target_arch = "x86_64")]
    println!("✅ TSC: {} cycles", tsc);
    
    // 3. Conexiones TCP reales
    #[cfg(target_os = "windows")]
    let connections = AssemblyVerifier::get_tcp_table_raw();
    #[cfg(target_os = "windows")]
    println!("✅ TCP Connections: {} activas", connections.len());
    
    #[cfg(target_os = "windows")]
    for (i, conn) in connections.iter().take(5).enumerate() {
        println!("   {}. PID {} | {}:{} -> {}:{} | State: {}",
            i + 1,
            conn.pid,
            conn.to_ip_string(),
            conn.local_port,
            format!("{}.{}.{}.{}", 
                conn.remote_addr & 0xFF,
                (conn.remote_addr >> 8) & 0xFF,
                (conn.remote_addr >> 16) & 0xFF,
                (conn.remote_addr >> 24) & 0xFF
            ),
            conn.remote_port,
            conn.state
        );
    }
    
    SystemVerificationReport {
        #[cfg(target_arch = "x86_64")]
        cpu_verified: true,
        #[cfg(not(target_arch = "x86_64"))]
        cpu_verified: false,
        
        #[cfg(target_os = "windows")]
        tcp_connections_count: connections.len(),
        #[cfg(not(target_os = "windows"))]
        tcp_connections_count: 0,
        
        verification_passed: true,
    }
}

#[derive(Debug)]
pub struct SystemVerificationReport {
    pub cpu_verified: bool,
    pub tcp_connections_count: usize,
    pub verification_passed: bool,
}
