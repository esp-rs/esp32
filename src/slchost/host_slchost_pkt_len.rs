#[doc = "Reader of register HOST_SLCHOST_PKT_LEN"]
pub type R = crate::R<u32, super::HOST_SLCHOST_PKT_LEN>;
#[doc = "Writer for register HOST_SLCHOST_PKT_LEN"]
pub type W = crate::W<u32, super::HOST_SLCHOST_PKT_LEN>;
#[doc = "Register HOST_SLCHOST_PKT_LEN `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_PKT_LEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_HOSTSLC0_LEN_CHECK`"]
pub type HOST_HOSTSLC0_LEN_CHECK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HOST_HOSTSLC0_LEN_CHECK`"]
pub struct HOST_HOSTSLC0_LEN_CHECK_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_HOSTSLC0_LEN_CHECK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | (((value as u32) & 0x0fff) << 20);
        self.w
    }
}
#[doc = "Reader of field `HOST_HOSTSLC0_LEN`"]
pub type HOST_HOSTSLC0_LEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HOST_HOSTSLC0_LEN`"]
pub struct HOST_HOSTSLC0_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_HOSTSLC0_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:31"]
    #[inline(always)]
    pub fn host_hostslc0_len_check(&self) -> HOST_HOSTSLC0_LEN_CHECK_R {
        HOST_HOSTSLC0_LEN_CHECK_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn host_hostslc0_len(&self) -> HOST_HOSTSLC0_LEN_R {
        HOST_HOSTSLC0_LEN_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 20:31"]
    #[inline(always)]
    pub fn host_hostslc0_len_check(&mut self) -> HOST_HOSTSLC0_LEN_CHECK_W {
        HOST_HOSTSLC0_LEN_CHECK_W { w: self }
    }
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn host_hostslc0_len(&mut self) -> HOST_HOSTSLC0_LEN_W {
        HOST_HOSTSLC0_LEN_W { w: self }
    }
}