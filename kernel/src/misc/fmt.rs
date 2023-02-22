pub struct NodesDisplay<'a>(pub &'a [crate::mem::node::NodeDescriptor]);

impl<'a> NodesDisplay<'a> {
    pub fn new(node: &'a crate::mem::node::NodeDescriptor) -> Self {
        Self(core::slice::from_ref(node))
    }
}

impl core::fmt::Display for NodesDisplay<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for (i, node) in self.0.iter().enumerate() {
            writeln!(f, "NODE #{} @ {:016X} - {:016X} (spanned: {}, present: {}), flags: {:#b}",
					 i, node.start_ppn << PAGE_SHIFT, node.start_ppn + node.spanned_pages << PAGE_SHIFT,
					 node.spanned_pages, node.present_pages, node.flags)?;

            let zone = &node.zone_normal;

            writeln!(f, "\tZONE #0 (NORMAL) @ {:016X} - {:016X} (spanned: {}, present: {}, managed: {}), flags: {:#b}",
					 zone.start_ppn << PAGE_SHIFT, zone.start_ppn + zone.spanned_pages << PAGE_SHIFT, zone.spanned_pages,
					 zone.present_pages, zone.managed_pages.load(Ordering::Relaxed), zone.flags)?;

            for (j, area) in zone.free_areas.iter().enumerate() {
                writeln!(f, "\t\tORDER #{:02} (size: {}): areas = {}", j, 1 << j, area.len.load(Ordering::Relaxed))?;

                let mut k = 0;
                let mut ptr = area.next;

                while let Some(page) = unsafe { ptr.as_ref() } {
                    let ppn = node.get_ppn(page);
                    writeln!(f, "\t\t\tAREA #{:04} @ {:016X} - {:016X}, flags: {:#b}, order = {}",
							 k, ppn << PAGE_SHIFT, ppn + (1 << j) << PAGE_SHIFT, page.flags.load(Ordering::Relaxed), page.idx)?;
                    k += 1;
                    ptr = page.next;

                    if k > 16 {
                        writeln!(f, "\t\t\t...")?;
                        break;
                    }
                }
            }
        }

		Ok(())
    }
}