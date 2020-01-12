#[doc = "Reader of register CPUSDIO_INT"]
pub type R = crate::R<u32, super::CPUSDIO_INT>;
#[doc = "Writer for register CPUSDIO_INT"]
pub type W = crate::W<u32, super::CPUSDIO_INT>;
#[doc = "Register CPUSDIO_INT `reset()`'s with value 0"]
impl crate::ResetValue for super::CPUSDIO_INT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDIO_INT`"]
pub type SDIO_INT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SDIO_INT`"]
pub struct SDIO_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - SDIO's extent GPIO0~31 interrupt"]
    #[inline(always)]
    pub fn sdio_int(&self) -> SDIO_INT_R {
        SDIO_INT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - SDIO's extent GPIO0~31 interrupt"]
    #[inline(always)]
    pub fn sdio_int(&mut self) -> SDIO_INT_W {
        SDIO_INT_W { w: self }
    }
}
