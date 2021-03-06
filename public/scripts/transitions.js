const searchContainerNode = document.getElementById('search');

window.onscroll = () => {
  if (window.pageYOffset > 0) {
    if (!searchContainerNode.classList.contains('search--with-shadow')) {
      searchContainerNode.classList.add('search--with-shadow');
    }
  } else {
    if (searchContainerNode.classList.contains('search--with-shadow')) {
      searchContainerNode.classList.remove('search--with-shadow');
    }
  }
};
