module Jekyll
    class APISectionBlockTag < Liquid::Block
        def initialize(tag_name, names, tokens)
            super
            names = names.split('|')
            @name = names[0]
            if names.length >= 2
                @alt_name = names[1]
            else
                @alt_name = @name
            end
        end

        def render(context)
            site = context.registers[:site]
            converter = site.getConverterImpl(Jekyll::Converters::Markdown)
            content = converter.convert(super.strip)
            "<section class='api-section'><h1 data-alt='#{@name}'>#{@alt_name}</h1>#{content}</section>".strip
        end
    end

    # Should be renamed so it's clear what it's actually used for (command syntax headers) TODO
    class APIBodyTag < Liquid::Block
        def initialize(tag_name, names, tokens)
            super
        end

        def render(context)
            site = context.registers[:site]
            converter = site.getConverterImpl(Jekyll::Converters::Markdown)
            content = "<p>#{super.gsub('<', '&lt;').gsub('>', '&gt;').strip.gsub(/\n([^\s])/, '</p><p>\1')}</p>"  #.gsub(/\n$/, '').gsub(/(?:\n\r?|\r\n?)/, '<br/>')
           "<section class='command-syntax'>#{content}</section>".strip
        end
    end

    # Shold be renamed so it's clear what this is used for (only the language switcher) TODO
    class APIUrlTag < Liquid::Block
        def initialize(tag_name, params, tokens)
            super
            @lang = params.gsub(/ /, '')
        end

        def render(context)
            '/'+super.gsub(/python|javascript|ruby/, @lang)
        end
    end
end

Liquid::Template.register_tag('apisection', Jekyll::APISectionBlockTag)
Liquid::Template.register_tag('apibody', Jekyll::APIBodyTag)
Liquid::Template.register_tag('apiurltag', Jekyll::APIUrlTag)
