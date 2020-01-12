#[doc = "Reader of register SDIO_SELECT"]
pub type R = crate::R<u32, super::SDIO_SELECT>;
#[doc = "Writer for register SDIO_SELECT"]
pub type W = crate::W<u32, super::SDIO_SELECT>;
#[doc = "Register SDIO_SELECT `reset()`'s with value 0"]
impl crate::ResetValue for super::SDIO_SELECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDIO_SEL`"]
pub type SDIO_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDIO_SEL`"]
pub struct SDIO_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SDIO PADS on/off control from outside"]
    #[inline(always)]
    pub fn sdio_sel(&self) -> SDIO_SEL_R {
        SDIO_SEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SDIO PADS on/off control from outside"]
    #[inline(always)]
    pub fn sdio_sel(&mut self) -> SDIO_SEL_W {
        SDIO_SEL_W { w: self }
    }
}
