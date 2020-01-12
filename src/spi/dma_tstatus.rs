#[doc = "Reader of register DMA_TSTATUS"]
pub type R = crate::R<u32, super::DMA_TSTATUS>;
#[doc = "Writer for register DMA_TSTATUS"]
pub type W = crate::W<u32, super::DMA_TSTATUS>;
#[doc = "Register DMA_TSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_TSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_IN_STATUS`"]
pub type DMA_IN_STATUS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DMA_IN_STATUS`"]
pub struct DMA_IN_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_IN_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - spi dma write data to memory status."]
    #[inline(always)]
    pub fn dma_in_status(&self) -> DMA_IN_STATUS_R {
        DMA_IN_STATUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - spi dma write data to memory status."]
    #[inline(always)]
    pub fn dma_in_status(&mut self) -> DMA_IN_STATUS_W {
        DMA_IN_STATUS_W { w: self }
    }
}
