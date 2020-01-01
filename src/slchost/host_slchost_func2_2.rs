#[doc = "Reader of register HOST_SLCHOST_FUNC2_2"]
pub type R = crate::R<u32, super::HOST_SLCHOST_FUNC2_2>;
#[doc = "Writer for register HOST_SLCHOST_FUNC2_2"]
pub type W = crate::W<u32, super::HOST_SLCHOST_FUNC2_2>;
#[doc = "Register HOST_SLCHOST_FUNC2_2 `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_FUNC2_2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLC_FUNC1_MDSTAT`"]
pub type HOST_SLC_FUNC1_MDSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_SLC_FUNC1_MDSTAT`"]
pub struct HOST_SLC_FUNC1_MDSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC_FUNC1_MDSTAT_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn host_slc_func1_mdstat(&self) -> HOST_SLC_FUNC1_MDSTAT_R {
        HOST_SLC_FUNC1_MDSTAT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn host_slc_func1_mdstat(&mut self) -> HOST_SLC_FUNC1_MDSTAT_W {
        HOST_SLC_FUNC1_MDSTAT_W { w: self }
    }
}
