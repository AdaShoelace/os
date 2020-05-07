## Linear page table

Number of Page table entries(PTE) = |address space| / page size  
eg. 16kb / 4kb = 4 pages => 4  
This means that we need to be able to index into 4 different offsets in the page table.  
We therefor need 2 bits to index into the page table. These are the top 2 bits of the virtual address.  
The remaining bits [12 in this case (`2¹² == 4096`)] gives us the byte offset into the page itself.  

So the translation would look like this:  
```
VPN_MASK = 0x3 << 12
OFFSET_MASK = 0xfff
OFFSET_SHIFT = 0x800

VPN = (VirtAddr & VPN_MASK) >> OFFSET_SHIFT
PTEAddr = PageTableBaseRegister + (VPN * sizeof(PTE))

PTE = access_memory(PTEAddr)

if PTE.validbit && PTE.can_access {
	offset = VirtAddr  & OFFSET_MASK	
	PhysAddr = (PTE.PFN << OFFSET_SHIFT) | offset
	register = access_memory(PhysAddr)
}
```
