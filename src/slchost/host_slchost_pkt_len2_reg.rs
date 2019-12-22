#[doc = "Reader of register HOST_SLCHOST_PKT_LEN2_REG"]
pub type R = crate::R<u32, super::HOST_SLCHOST_PKT_LEN2_REG>;
#[doc = "Writer for register HOST_SLCHOST_PKT_LEN2_REG"]
pub type W = crate::W<u32, super::HOST_SLCHOST_PKT_LEN2_REG>;
#[doc = "Register HOST_SLCHOST_PKT_LEN2_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_PKT_LEN2_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_HOSTSLC0_LEN2`"]
pub type HOST_HOSTSLC0_LEN2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HOST_HOSTSLC0_LEN2`"]
pub struct HOST_HOSTSLC0_LEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_HOSTSLC0_LEN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn host_hostslc0_len2(&self) -> HOST_HOSTSLC0_LEN2_R {
        HOST_HOSTSLC0_LEN2_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn host_hostslc0_len2(&mut self) -> HOST_HOSTSLC0_LEN2_W {
        HOST_HOSTSLC0_LEN2_W { w: self }
    }
}
