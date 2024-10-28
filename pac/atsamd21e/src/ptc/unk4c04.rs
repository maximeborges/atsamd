#[doc = "Register `UNK4C04` reader"]
pub type R = crate::R<Unk4c04Spec>;
#[doc = "Register `UNK4C04` writer"]
pub type W = crate::W<Unk4c04Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Unknown Register 0x42004C04\n\nYou can [`read`](crate::Reg::read) this register and get [`unk4c04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unk4c04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Unk4c04Spec;
impl crate::RegisterSpec for Unk4c04Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`unk4c04::R`](R) reader structure"]
impl crate::Readable for Unk4c04Spec {}
#[doc = "`write(|w| ..)` method takes [`unk4c04::W`](W) writer structure"]
impl crate::Writable for Unk4c04Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UNK4C04 to value 0"]
impl crate::Resettable for Unk4c04Spec {
    const RESET_VALUE: u8 = 0;
}
