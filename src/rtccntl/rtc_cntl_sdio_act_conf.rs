#[doc = "Reader of register RTC_CNTL_SDIO_ACT_CONF"]
pub type R = crate::R<u32, super::RTC_CNTL_SDIO_ACT_CONF>;
#[doc = "Writer for register RTC_CNTL_SDIO_ACT_CONF"]
pub type W = crate::W<u32, super::RTC_CNTL_SDIO_ACT_CONF>;
#[doc = "Register RTC_CNTL_SDIO_ACT_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_SDIO_ACT_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_SDIO_ACT_DNUM`"]
pub type RTC_CNTL_SDIO_ACT_DNUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RTC_CNTL_SDIO_ACT_DNUM`"]
pub struct RTC_CNTL_SDIO_ACT_DNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SDIO_ACT_DNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 22)) | (((value as u32) & 0x03ff) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_act_dnum(&self) -> RTC_CNTL_SDIO_ACT_DNUM_R {
        RTC_CNTL_SDIO_ACT_DNUM_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_act_dnum(&mut self) -> RTC_CNTL_SDIO_ACT_DNUM_W {
        RTC_CNTL_SDIO_ACT_DNUM_W { w: self }
    }
}
