#[doc = "Reader of register HOST_SLC1HOST_TOKEN_WDATA"]
pub type R = crate::R<u32, super::HOST_SLC1HOST_TOKEN_WDATA>;
#[doc = "Writer for register HOST_SLC1HOST_TOKEN_WDATA"]
pub type W = crate::W<u32, super::HOST_SLC1HOST_TOKEN_WDATA>;
#[doc = "Register HOST_SLC1HOST_TOKEN_WDATA `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLC1HOST_TOKEN_WDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLC1HOST_TOKEN1_WD`"]
pub type HOST_SLC1HOST_TOKEN1_WD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HOST_SLC1HOST_TOKEN1_WD`"]
pub struct HOST_SLC1HOST_TOKEN1_WD_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC1HOST_TOKEN1_WD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLC1HOST_TOKEN0_WD`"]
pub type HOST_SLC1HOST_TOKEN0_WD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HOST_SLC1HOST_TOKEN0_WD`"]
pub struct HOST_SLC1HOST_TOKEN0_WD_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC1HOST_TOKEN0_WD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn host_slc1host_token1_wd(&self) -> HOST_SLC1HOST_TOKEN1_WD_R {
        HOST_SLC1HOST_TOKEN1_WD_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn host_slc1host_token0_wd(&self) -> HOST_SLC1HOST_TOKEN0_WD_R {
        HOST_SLC1HOST_TOKEN0_WD_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn host_slc1host_token1_wd(&mut self) -> HOST_SLC1HOST_TOKEN1_WD_W {
        HOST_SLC1HOST_TOKEN1_WD_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn host_slc1host_token0_wd(&mut self) -> HOST_SLC1HOST_TOKEN0_WD_W {
        HOST_SLC1HOST_TOKEN0_WD_W { w: self }
    }
}
