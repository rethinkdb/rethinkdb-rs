module Jekyll
  module PageTitleFilter
    def page_title(input)
      "<h1 class='title'>#{input}</h1>"
    end
  end
end

Liquid::Template.register_filter(Jekyll::PageTitleFilter)
