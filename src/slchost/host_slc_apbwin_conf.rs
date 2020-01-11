#[doc = "Reader of register HOST_SLC_APBWIN_CONF"]
pub type R = crate::R<u32, super::HOST_SLC_APBWIN_CONF>;
#[doc = "Writer for register HOST_SLC_APBWIN_CONF"]
pub type W = crate::W<u32, super::HOST_SLC_APBWIN_CONF>;
#[doc = "Register HOST_SLC_APBWIN_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLC_APBWIN_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLC_APBWIN_START`"]
pub type HOST_SLC_APBWIN_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_SLC_APBWIN_START`"]
pub struct HOST_SLC_APBWIN_START_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC_APBWIN_START_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLC_APBWIN_WR`"]
pub type HOST_SLC_APBWIN_WR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_SLC_APBWIN_WR`"]
pub struct HOST_SLC_APBWIN_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC_APBWIN_WR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLC_APBWIN_ADDR`"]
pub type HOST_SLC_APBWIN_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HOST_SLC_APBWIN_ADDR`"]
pub struct HOST_SLC_APBWIN_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC_APBWIN_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | ((value as u32) & 0x0fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn host_slc_apbwin_start(&self) -> HOST_SLC_APBWIN_START_R {
        HOST_SLC_APBWIN_START_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn host_slc_apbwin_wr(&self) -> HOST_SLC_APBWIN_WR_R {
        HOST_SLC_APBWIN_WR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn host_slc_apbwin_addr(&self) -> HOST_SLC_APBWIN_ADDR_R {
        HOST_SLC_APBWIN_ADDR_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn host_slc_apbwin_start(&mut self) -> HOST_SLC_APBWIN_START_W {
        HOST_SLC_APBWIN_START_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn host_slc_apbwin_wr(&mut self) -> HOST_SLC_APBWIN_WR_W {
        HOST_SLC_APBWIN_WR_W { w: self }
    }
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn host_slc_apbwin_addr(&mut self) -> HOST_SLC_APBWIN_ADDR_W {
        HOST_SLC_APBWIN_ADDR_W { w: self }
    }
}
