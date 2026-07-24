<script setup lang="ts">
  import { ref, onMounted } from 'vue';
  import { Gender } from '@/domain/entities/gender';
  import { GenderService } from '@/application/services/GenderService';
  import { GenderRepository } from '@/infrastructure/repositories/GenderRepository';

  const genderService = new GenderService(new GenderRepository());

  const genders = ref<Gender[]>([])

  onMounted(async() => {
    try {
      genders.value = await genderService.getGenderByName("a", 0, 20);
    } catch (error) {
      console.error("Erro nas requisições: ", error)
    }
    
  })
</script>

<template>
  <div>
    <h1>Página Inicial da aplicação</h1>
    <p>Vou realizar alguns testes</p>
    <p>Para ter uma ideia</p>
    <p>do que está acontecendo</p>

    <li v-for="gender in genders">
      {{ gender.name }}
    </li>

    <p>
      Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla sit amet molestie odio. Fusce dignissim ligula fringilla ultricies pulvinar. Aenean sagittis lobortis orci sed facilisis. Sed metus ipsum, tempus eu velit eget, porttitor placerat quam. Duis congue lacus turpis, non dictum mauris efficitur eget. Ut semper tempor pharetra. Cras imperdiet felis eget mauris tempor pellentesque. Nullam bibendum mollis lorem, vestibulum tristique urna tincidunt non. Praesent lectus urna, lacinia id purus dictum, dictum congue lectus. Donec molestie cursus tempus. 
    </p>

    <p>
      Sed blandit ex justo, non tincidunt dui eleifend ac. Pellentesque ut lacinia tortor. Etiam efficitur odio vel maximus accumsan. Praesent blandit lacus in lectus pulvinar eleifend. Nullam malesuada feugiat quam, sed feugiat mi. Donec bibendum at purus vel posuere. Suspendisse sit amet velit vitae est gravida tristique finibus non nunc. Donec nec ultrices nisl. Proin vulputate id quam eget tincidunt. Aliquam porta lorem ut auctor viverra. Morbi pellentesque aliquam dui, finibus mattis mi cursus id. Praesent molestie tellus massa, fringilla efficitur sapien vulputate a. Donec feugiat urna at metus sodales dignissim. Nullam viverra odio eget turpis aliquam, id tempus urna vestibulum.
    </p>

    <p>
      Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Morbi quis dui posuere, gravida augue mattis, rhoncus odio. Mauris risus nisi, tincidunt a egestas quis, dictum sed quam. Vestibulum ex dolor, condimentum at elementum at, hendrerit ac metus. Nulla facilisi. Integer iaculis, nisl sed egestas fermentum, nisi leo scelerisque justo, eget pretium felis risus vel sem. Sed sed est sed lectus rhoncus cursus. Duis dictum molestie magna ut pretium. Pellentesque eu lacus vitae dui tristique aliquam ac nec tellus. Vestibulum sodales hendrerit gravida. Sed cursus sem non nibh tristique, ac dictum orci condimentum. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Sed in felis consectetur, elementum lorem tincidunt, tincidunt nunc. Praesent tempus nisl eget leo imperdiet vulputate.
    </p>

    <p>
      Ut rutrum fringilla pulvinar. Ut erat eros, gravida sed erat vitae, facilisis pellentesque mauris. Interdum et malesuada fames ac ante ipsum primis in faucibus. Proin non tellus sed sem bibendum rutrum. Cras quis orci at nunc imperdiet porta. Duis eu turpis neque. Quisque facilisis consequat enim, ultrices pulvinar massa elementum sit amet. Sed molestie elit nec egestas auctor. Suspendisse ex lectus, interdum ut sagittis et, placerat ut elit. Vivamus ut finibus nulla, id cursus dolor. Fusce a lorem ac nisi pulvinar fermentum. Sed id nunc at elit volutpat tempus. Nulla vel turpis pharetra, tincidunt velit non, lacinia dolor. In consectetur iaculis risus, non tempor quam convallis in.
    </p>

    <p>
      Suspendisse potenti. Etiam a lorem nibh. Sed egestas nunc placerat ligula egestas molestie. Nunc vestibulum consequat blandit. Ut semper sit amet velit non laoreet. Vestibulum volutpat dui ac orci bibendum feugiat. Proin eu velit elit. Aliquam erat volutpat. Nam sed quam ex. Sed nec quam fringilla, facilisis turpis eget, fermentum turpis. Quisque ullamcorper augue eget diam tincidunt hendrerit. Integer tortor mauris, posuere non faucibus vitae, hendrerit ac mi. Phasellus tristique euismod metus et porttitor.
    </p>
  </div>
</template>

<style scoped>
</style>