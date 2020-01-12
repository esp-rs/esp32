#[doc = "Reader of register HOST_INF_SEL"]
pub type R = crate::R<u32, super::HOST_INF_SEL>;
#[doc = "Writer for register HOST_INF_SEL"]
pub type W = crate::W<u32, super::HOST_INF_SEL>;
#[doc = "Register HOST_INF_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_INF_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LINK_DEVICE_SEL`"]
pub type LINK_DEVICE_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LINK_DEVICE_SEL`"]
pub struct LINK_DEVICE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_DEVICE_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `PERI_IO_SWAP`"]
pub type PERI_IO_SWAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PERI_IO_SWAP`"]
pub struct PERI_IO_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_IO_SWAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn link_device_sel(&self) -> LINK_DEVICE_SEL_R {
        LINK_DEVICE_SEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn peri_io_swap(&self) -> PERI_IO_SWAP_R {
        PERI_IO_SWAP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn link_device_sel(&mut self) -> LINK_DEVICE_SEL_W {
        LINK_DEVICE_SEL_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn peri_io_swap(&mut self) -> PERI_IO_SWAP_W {
        PERI_IO_SWAP_W { w: self }
    }
}
